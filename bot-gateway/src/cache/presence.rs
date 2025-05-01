use std::time::Duration;

use randy_model::{
    gateway::{
        event::Event,
        payload::incoming::PresenceUpdate,
        presence::{ClientStatus, Presence, Status, UserOrId},
    },
    id::{Id, marker::UserMarker},
};
use redlight::{
    RedisCache,
    config::{CacheConfig, Cacheable, ICachedPresence, Ignore},
    error::CacheError,
    rkyv_util::{id::IdRkyv, presence::StatusRkyv},
};
use rkyv::{Archive, Serialize, rancor::Source, ser::writer::Buffer, util::Align, with::Map};

use super::user::user;
use crate::pool;

#[derive(Archive, Serialize)]
pub struct CachedPresence {
    #[rkyv(with = Map<StatusRkyv>)]
    desktop_status: Option<Status>,
    #[rkyv(with = IdRkyv)]
    user_id: Id<UserMarker>,
}

impl<'a> ICachedPresence<'a> for CachedPresence {
    fn from_presence(presence: &'a Presence) -> Self {
        Self {
            desktop_status: presence.client_status.desktop,
            user_id: presence.user.id(),
        }
    }
}

impl Cacheable for CachedPresence {
    type Bytes = [u8; 16];

    fn expire() -> Option<Duration> {
        None
    }

    fn serialize_one<E: Source>(&self) -> Result<Self::Bytes, E> {
        let mut bytes = Align([0_u8; 16]);
        rkyv::api::high::to_bytes_in(self, Buffer::from(&mut *bytes))?;

        Ok(bytes.0)
    }
}
