#![doc = include_str!("../README.md")]
#![warn(
    clippy::missing_const_for_fn,
    clippy::missing_docs_in_private_items,
    clippy::pedantic,
    missing_docs,
    unsafe_code
)]
#![allow(
    clippy::module_name_repetitions,
    clippy::must_use_candidate,
    clippy::unnecessary_wraps
)]

pub mod headers;
pub mod in_memory;
pub mod request;
pub mod ticket;

pub use self::{
    headers::RatelimitHeaders,
    in_memory::InMemoryRatelimiter,
    request::{Method, Path},
};

use self::ticket::{TicketReceiver, TicketSender};
use std::{
    error::Error,
    fmt::Debug,
    future::Future,
    pin::Pin,
    time::{Duration, Instant},
};

/// A bucket containing ratelimiting information for a [`Path`].
pub struct Bucket {
    /// Total number of tickets allotted in a cycle.
    limit: u64,
    /// Number of tickets remaining.
    remaining: u64,
    /// Duration after [`Self::started_at`] time the bucket will refresh.
    reset_after: Duration,
    /// When the bucket's ratelimit refresh countdown started.
    started_at: Option<Instant>,
}

impl Bucket {
    /// Create a representation of a ratelimiter bucket.
    ///
    /// Buckets are returned by ratelimiters via [`Ratelimiter::bucket`] method.
    /// Its primary use is for informational purposes, including information
    /// such as the [number of remaining tickets][`Self::limit`] or determining
    /// how much time remains
    /// [until the bucket interval resets][`Self::time_remaining`].
    #[must_use]
    pub const fn new(
        limit: u64,
        remaining: u64,
        reset_after: Duration,
        started_at: Option<Instant>,
    ) -> Self {
        Self {
            limit,
            remaining,
            reset_after,
            started_at,
        }
    }

    /// Total number of tickets allotted in a cycle.
    #[must_use]
    pub const fn limit(&self) -> u64 {
        self.limit
    }

    /// Number of tickets remaining.
    #[must_use]
    pub const fn remaining(&self) -> u64 {
        self.remaining
    }

    /// Duration after the [`Self::started_at`] time the bucket will
    /// refresh.
    #[must_use]
    pub const fn reset_after(&self) -> Duration {
        self.reset_after
    }

    /// When the bucket's ratelimit refresh countdown started.
    #[must_use]
    pub const fn started_at(&self) -> Option<Instant> {
        self.started_at
    }

    /// How long until the bucket will refresh.
    ///
    /// May return `None` if the refresh timer has not been started yet or
    /// the bucket has already refreshed.
    #[must_use]
    pub fn time_remaining(&self) -> Option<Duration> {
        let reset_at = self.started_at? + self.reset_after;

        reset_at.checked_duration_since(Instant::now())
    }
}

/// A generic error type that implements [`Error`].
pub type GenericError = Box<dyn Error + Send + Sync>;

/// Future returned by [`Ratelimiter::bucket`].
pub type GetBucketFuture =
    Pin<Box<dyn Future<Output = Result<Option<Bucket>, GenericError>> + Send + 'static>>;

/// Future returned by [`Ratelimiter::is_globally_locked`].
pub type IsGloballyLockedFuture =
    Pin<Box<dyn Future<Output = Result<bool, GenericError>> + Send + 'static>>;

/// Future returned by [`Ratelimiter::has`].
pub type HasBucketFuture =
    Pin<Box<dyn Future<Output = Result<bool, GenericError>> + Send + 'static>>;

/// Future returned by [`Ratelimiter::ticket`].
pub type GetTicketFuture =
    Pin<Box<dyn Future<Output = Result<TicketReceiver, GenericError>> + Send + 'static>>;

/// Future returned by [`Ratelimiter::wait_for_ticket`].
pub type WaitForTicketFuture =
    Pin<Box<dyn Future<Output = Result<TicketSender, GenericError>> + Send + 'static>>;

/// An implementation of a ratelimiter for the Discord REST API.
///
/// A default implementation can be found in [`InMemoryRatelimiter`].
///
/// All operations are asynchronous to allow for custom implementations to
/// use different storage backends, for example databases.
///
/// Ratelimiters should keep track of two kids of ratelimits:
/// * The global ratelimit status
/// * [`Path`]-specific ratelimits
///
/// To do this, clients utilizing a ratelimiter will send back response
/// ratelimit headers via a [`TicketSender`].
///
/// The ratelimiter itself will hand a [`TicketReceiver`] to the caller
/// when a ticket is being requested.
pub trait Ratelimiter: Debug + Send + Sync {
    /// Retrieve the basic information of the bucket for a given path.
    fn bucket(&self, path: &Path) -> GetBucketFuture;

    /// Whether the ratelimiter is currently globally locked.
    fn is_globally_locked(&self) -> IsGloballyLockedFuture;

    /// Determine if the ratelimiter has a bucket for the given path.
    fn has(&self, path: &Path) -> HasBucketFuture;

    /// Retrieve a ticket to know when to send a request.
    /// The provided future will be ready when a ticket in the bucket is
    /// available. Tickets are ready in order of retrieval.
    fn ticket(&self, path: Path) -> GetTicketFuture;

    /// Retrieve a ticket to send a request.
    /// Other than [`Self::ticket`], this method will return
    /// a [`TicketSender`].
    ///
    /// This is identical to calling [`Self::ticket`] and then
    /// awaiting the [`TicketReceiver`].
    fn wait_for_ticket(&self, path: Path) -> WaitForTicketFuture {
        let get_ticket = self.ticket(path);
        Box::pin(async move {
            match get_ticket.await {
                Ok(rx) => rx.await.map_err(From::from),
                Err(e) => Err(e),
            }
        })
    }
}
