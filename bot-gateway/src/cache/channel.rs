use std::{
    fmt::{Debug, Formatter, Result as FmtResult},
    ops::Deref,
    time::Duration,
};

use randy_model::{
    channel::{Channel, ChannelFlags, ChannelType, VideoQualityMode},
    gateway::{
        event::Event,
        payload::incoming::{ChannelCreate, ChannelPinsUpdate},
    },
    id::{Id, marker::ChannelMarker},
    util::{ImageHash, Timestamp},
};
use redlight::{
    CachedArchive, RedisCache,
    config::{CacheConfig, Cacheable, ICachedChannel, Ignore},
    error::CacheError,
    rkyv_util::{
        id::{IdRkyv, IdRkyvMap},
        timestamp::{ArchivedTimestamp, TimestampRkyv},
    },
};
use rkyv::{
    Archive, Archived, Serialize,
    option::ArchivedOption,
    rancor::Source,
    util::AlignedVec,
    with::{InlineAsBox, Map},
};

#[derive(Archive, Serialize)]
pub struct CachedChannel<'a> {
    #[rkyv(with = Map<InlineAsBox>)]
    name: Option<&'a str>,
    #[rkyv(with = IdRkyv)]
    id: Id<ChannelMarker>,
    kind: u8,
    #[rkyv(with = Map<TimestampRkyv>)]
    last_pin_timestamp: Option<Timestamp>,
    #[rkyv(with = IdRkyvMap)]
    parent_id: Option<Id<ChannelMarker>>,
}

impl Debug for ArchivedCachedChannel<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        f.debug_struct("ArchivedCachedChannel")
            .field("name", &self.name.as_deref())
            .field("id", &self.id)
            .field("kind", &self.kind)
            .field("last_pin_timestamp", &self.last_pin_timestamp)
            .field("parent_id", &self.parent_id)
            .finish()
    }
}

impl<'a> ICachedChannel<'a> for CachedChannel<'a> {
    fn from_channel(channel: &'a Channel) -> Self {
        Self {
            name: channel.name.as_deref(),
            id: channel.id,
            kind: channel.kind.into(),
            last_pin_timestamp: channel.last_pin_timestamp,
            parent_id: channel.parent_id,
        }
    }

    fn on_pins_update<E: Source>()
    -> Option<fn(&mut CachedArchive<Archived<Self>>, &ChannelPinsUpdate) -> Result<(), E>> {
        Some(|value, update| {
            value.update_archive(|sealed| {
                if let Some(new_timestamp) = update.last_pin_timestamp {
                    rkyv::munge::munge! {
                        let ArchivedCachedChannel { last_pin_timestamp, .. } = sealed
                    };

                    // Cannot mutate from `Some` to `None` or vice versa so we
                    // just update `Some` values
                    if let Some(mut last_pin_timestamp) =
                        ArchivedOption::as_seal(last_pin_timestamp)
                    {
                        *last_pin_timestamp = ArchivedTimestamp::new(&new_timestamp);
                    }
                }
            });

            Ok(())
        })
    }
}

impl Cacheable for CachedChannel<'_> {
    type Bytes = AlignedVec<8>;

    fn expire() -> Option<Duration> {
        None
    }

    fn serialize_one<E: Source>(&self) -> Result<Self::Bytes, E> {
        rkyv::api::high::to_bytes_in(self, AlignedVec::<8>::new())
    }

    // we don't update by deserializing so a `serialize_into` impl is not
    // necessary
}

impl PartialEq<Channel> for ArchivedCachedChannel<'_> {
    fn eq(&self, other: &Channel) -> bool {
        let Self {
            name,
            id,
            kind,
            last_pin_timestamp,
            parent_id,
        } = self;

        name.as_deref() == other.name.as_deref()
            && *id == other.id
            && *kind == u8::from(other.kind)
            && *last_pin_timestamp
                == other
                    .last_pin_timestamp
                    .as_ref()
                    .map(ArchivedTimestamp::new)
            && parent_id.as_ref().copied().map(Id::from) == other.parent_id
    }
}
