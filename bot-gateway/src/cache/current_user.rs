use std::time::Duration;

use randy_model::{
    gateway::{event::Event, payload::incoming::UserUpdate},
    id::{Id, marker::UserMarker},
    user::{CurrentUser, PremiumType, UserFlags},
    util::ImageHash,
};
use redlight::{
    RedisCache,
    config::{CacheConfig, Cacheable, ICachedCurrentUser, Ignore},
    error::CacheError,
    rkyv_util::id::IdRkyv,
};
use rkyv::{
    Archive, Serialize,
    rancor::Source,
    util::AlignedVec,
    with::{InlineAsBox, Map},
};

#[derive(Archive, Serialize)]
#[rkyv(remote = ImageHash)]
#[expect(unused)]
pub struct ImageHashRkyv {
    #[rkyv(getter = get_animated)]
    animated: bool,
    #[rkyv(getter = get_bytes)]
    bytes: [u8; 16],
}

impl ImageHashRkyv {
    fn get_animated(&self) -> bool {
        self.animated
    }

    fn get_bytes(&self) -> [u8; 16] {
        self.bytes
    }
}

#[derive(Archive, Serialize)]
pub struct CachedCurrentUser<'a> {
    #[rkyv(with = Map<ImageHashRkyv>)]
    avatar: Option<ImageHash>,
    #[rkyv(with = InlineAsBox)]
    name: &'a str,
    #[rkyv(with = IdRkyv)]
    id: Id<UserMarker>,
}

impl<'a> ICachedCurrentUser<'a> for CachedCurrentUser<'a> {
    fn from_current_user(current_user: &'a CurrentUser) -> Self {
        Self {
            avatar: current_user.avatar,
            name: &current_user.name,
            id: current_user.id,
        }
    }
}

impl Cacheable for CachedCurrentUser<'_> {
    type Bytes = AlignedVec;

    fn expire() -> Option<Duration> {
        None
    }

    fn serialize_one<E: Source>(&self) -> Result<Self::Bytes, E> {
        rkyv::to_bytes(self)
    }
}
