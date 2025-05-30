use crate::{
    client::Client,
    error::Error,
    request::{Request, TryIntoRequest},
    response::{marker::EmptyBody, Response, ResponseFuture},
    routing::Route,
};
use std::future::IntoFuture;
use randy_model::id::{marker::ChannelMarker, Id};

/// Delete the stage instance of a stage channel.
///
/// Requires the user to be a moderator of the stage channel.
#[must_use = "requests must be configured and executed"]
pub struct DeleteStageInstance<'a> {
    channel_id: Id<ChannelMarker>,
    http: &'a Client,
}

impl<'a> DeleteStageInstance<'a> {
    pub(crate) const fn new(http: &'a Client, channel_id: Id<ChannelMarker>) -> Self {
        Self { channel_id, http }
    }
}

impl IntoFuture for DeleteStageInstance<'_> {
    type Output = Result<Response<EmptyBody>, Error>;

    type IntoFuture = ResponseFuture<EmptyBody>;

    fn into_future(self) -> Self::IntoFuture {
        let http = self.http;

        match self.try_into_request() {
            Ok(request) => http.request(request),
            Err(source) => ResponseFuture::error(source),
        }
    }
}

impl TryIntoRequest for DeleteStageInstance<'_> {
    fn try_into_request(self) -> Result<Request, Error> {
        Ok(Request::from_route(&Route::DeleteStageInstance {
            channel_id: self.channel_id.get(),
        }))
    }
}
