use std::fmt::{Debug, Formatter, Result as FmtResult};

use randy_model::id::Id;
use rkyv::{rancor::Source, Archive, Archived};
use tracing::{instrument, trace};

use super::{
    impls::{
        channel::{ChannelKey, ChannelMetaKey},
        emoji::{EmojiKey, EmojiMetaKey},
        guild::{GuildKey, GuildMetaKey},
        integration::{IntegrationKey, IntegrationMetaKey},
        member::{MemberKey, MemberMetaKey},
        message::{MessageKey, MessageMetaKey},
        presence::{PresenceKey, PresenceMetaKey},
        role::{RoleKey, RoleMetaKey},
        scheduled_event::{ScheduledEventKey, ScheduledEventMetaKey},
        stage_instance::{StageInstanceKey, StageInstanceMetaKey},
        sticker::{StickerKey, StickerMetaKey},
        user::{UserKey, UserMetaKey},
        voice_state::{VoiceStateKey, VoiceStateMetaKey},
    },
    pipe::Pipe,
};
use crate::{
    config::CheckedArchived,
    error::ExpireError,
    key::RedisKey,
    redis::{DedicatedConnection, Pipeline},
};

pub(crate) enum MetaKey {
    Channel(ChannelMetaKey),
    Emoji(EmojiMetaKey),
    Guild(GuildMetaKey),
    Integration(IntegrationMetaKey),
    Member(MemberMetaKey),
    Message(MessageMetaKey),
    Presence(PresenceMetaKey),
    Role(RoleMetaKey),
    ScheduledEvent(ScheduledEventMetaKey),
    StageInstance(StageInstanceMetaKey),
    Sticker(StickerMetaKey),
    User(UserMetaKey),
    VoiceState(VoiceStateMetaKey),
}

impl MetaKey {
    pub(crate) fn parse<'a>(split: &mut impl Iterator<Item = &'a [u8]>) -> Option<Self> {
        match split.next() {
            Some(ChannelKey::PREFIX) => IMetaKey::parse(split).map(Self::Channel),
            Some(EmojiKey::PREFIX) => IMetaKey::parse(split).map(Self::Emoji),
            Some(GuildKey::PREFIX) => IMetaKey::parse(split).map(Self::Guild),
            Some(IntegrationKey::PREFIX) => IMetaKey::parse(split).map(Self::Integration),
            Some(MemberKey::PREFIX) => IMetaKey::parse(split).map(Self::Member),
            Some(MessageKey::PREFIX) => IMetaKey::parse(split).map(Self::Message),
            Some(PresenceKey::PREFIX) => IMetaKey::parse(split).map(Self::Presence),
            Some(RoleKey::PREFIX) => IMetaKey::parse(split).map(Self::Role),
            Some(ScheduledEventKey::PREFIX) => IMetaKey::parse(split).map(Self::ScheduledEvent),
            Some(StageInstanceKey::PREFIX) => IMetaKey::parse(split).map(Self::StageInstance),
            Some(StickerKey::PREFIX) => IMetaKey::parse(split).map(Self::Sticker),
            Some(UserKey::PREFIX) => IMetaKey::parse(split).map(Self::User),
            Some(VoiceStateKey::PREFIX) => IMetaKey::parse(split).map(Self::VoiceState),
            Some(_) | None => None,
        }
    }

    #[instrument(level = "trace", skip(conn, pipe))]
    pub(crate) async fn handle_expire(
        self,
        conn: &mut DedicatedConnection,
        pipe: &mut Pipeline,
    ) -> Result<(), ExpireError> {
        match self {
            MetaKey::Channel(meta) => {
                let key = meta.redis_key();

                let Some(bytes) = Self::fetch_bytes(conn, pipe, key).await? else {
                    return Ok(());
                };

                let archived = <ChannelMetaKey as HasArchived>::Meta::as_archive(&bytes)?;
                meta.handle_archived(pipe, archived);
                meta.handle_expire(pipe);
            }
            MetaKey::Emoji(meta) => {
                let key = meta.redis_key();

                let Some(bytes) = Self::fetch_bytes(conn, pipe, key).await? else {
                    return Ok(());
                };

                let archived = <EmojiMetaKey as HasArchived>::Meta::as_archive(&bytes)?;
                meta.handle_archived(pipe, archived);
                meta.handle_expire(pipe);
            }
            MetaKey::Guild(meta) => {
                meta.handle_expire(pipe);
                meta.async_handle_expire(pipe, conn).await?;
            }
            MetaKey::Integration(meta) => meta.handle_expire(pipe),
            MetaKey::Member(meta) => {
                meta.handle_expire(pipe);
                meta.async_handle_expire(pipe, conn).await?;
            }
            MetaKey::Message(meta) => {
                let key = meta.redis_key();

                let Some(bytes) = Self::fetch_bytes(conn, pipe, key).await? else {
                    return Ok(());
                };

                let archived = <MessageMetaKey as HasArchived>::Meta::as_archive(&bytes)?;
                meta.handle_archived(pipe, archived);
                meta.handle_expire(pipe);
            }
            MetaKey::Presence(meta) => meta.handle_expire(pipe),
            MetaKey::Role(meta) => {
                let key = meta.redis_key();

                let Some(bytes) = Self::fetch_bytes(conn, pipe, key).await? else {
                    return Ok(());
                };

                let archived = <RoleMetaKey as HasArchived>::Meta::as_archive(&bytes)?;
                meta.handle_archived(pipe, archived);
                meta.handle_expire(pipe);
            }
            MetaKey::ScheduledEvent(meta) => {
                let key = meta.redis_key();

                let Some(bytes) = Self::fetch_bytes(conn, pipe, key).await? else {
                    return Ok(());
                };

                let archived = <ScheduledEventMetaKey as HasArchived>::Meta::as_archive(&bytes)?;
                meta.handle_archived(pipe, archived);
                meta.handle_expire(pipe);
            }
            MetaKey::StageInstance(meta) => {
                let key = meta.redis_key();

                let Some(bytes) = Self::fetch_bytes(conn, pipe, key).await? else {
                    return Ok(());
                };

                let archived = <StageInstanceMetaKey as HasArchived>::Meta::as_archive(&bytes)?;
                meta.handle_archived(pipe, archived);
                meta.handle_expire(pipe);
            }
            MetaKey::Sticker(meta) => {
                let key = meta.redis_key();

                let Some(bytes) = Self::fetch_bytes(conn, pipe, key).await? else {
                    return Ok(());
                };

                let archived = <StickerMetaKey as HasArchived>::Meta::as_archive(&bytes)?;
                meta.handle_archived(pipe, archived);
                meta.handle_expire(pipe);
            }
            MetaKey::User(meta) => meta.handle_expire(pipe),
            MetaKey::VoiceState(meta) => meta.handle_expire(pipe),
        }

        trace!(piped = pipe.cmd_iter().count());

        Ok(())
    }

    async fn fetch_bytes(
        conn: &mut DedicatedConnection,
        pipe: &mut Pipeline,
        key: impl RedisKey,
    ) -> Result<Option<Vec<u8>>, ExpireError> {
        debug_assert_eq!(pipe.cmd_iter().count(), 0);

        let res = pipe
            .get_del(key)
            .query_async::<_, Option<Vec<u8>>>(conn)
            .await
            .map(|opt| opt.filter(|bytes| !bytes.is_empty()))
            .map_err(ExpireError::GetMeta);

        pipe.clear();

        res
    }
}

impl Debug for MetaKey {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match self {
            Self::Channel(meta) => Debug::fmt(meta, f),
            Self::Emoji(meta) => Debug::fmt(meta, f),
            Self::Guild(meta) => Debug::fmt(meta, f),
            Self::Integration(meta) => Debug::fmt(meta, f),
            Self::Member(meta) => Debug::fmt(meta, f),
            Self::Message(meta) => Debug::fmt(meta, f),
            Self::Presence(meta) => Debug::fmt(meta, f),
            Self::Role(meta) => Debug::fmt(meta, f),
            Self::ScheduledEvent(meta) => Debug::fmt(meta, f),
            Self::StageInstance(meta) => Debug::fmt(meta, f),
            Self::Sticker(meta) => Debug::fmt(meta, f),
            Self::User(meta) => Debug::fmt(meta, f),
            Self::VoiceState(meta) => Debug::fmt(meta, f),
        }
    }
}

/// All the data given by a [`RedisKey`] alone.
///
/// Created from an expire payload. If additional data is required to perform
/// the expire cleanup, implement [`HasArchived`].
pub(crate) trait IMetaKey: Sized {
    /// Parse from an expire payload.
    fn parse<'a>(split: &mut impl Iterator<Item = &'a [u8]>) -> Option<Self>;

    /// What to do after the payload has been parsed.
    fn handle_expire(&self, pipe: &mut Pipeline);
}

/// Specifies that a [`IMetaKey`] has additional archived data.
pub(crate) trait HasArchived: Sized {
    type Meta: IMeta<Self>;

    /// The [`RedisKey`] to gather the additional data.
    fn redis_key(&self) -> impl RedisKey;

    /// What to do after the additional data has been retrieved.
    fn handle_archived(&self, pipe: &mut Pipeline, archived: &Archived<Self::Meta>);
}

/// Additional data for a [`IMetaKey`] that gets archived in the cache.
pub(crate) trait IMeta<Key: HasArchived>:
    Archive<Archived: CheckedArchived> + Sized
{
    type Bytes: AsRef<[u8]>;

    fn to_bytes<E: Source>(&self) -> Result<Self::Bytes, E>;

    /// Serialize and store this data in the cache.
    fn store<C, E: Source>(&self, pipe: &mut Pipe<'_, C>, key: Key) -> Result<(), E> {
        let bytes = self.to_bytes()?;
        let key = key.redis_key();
        pipe.set(key, bytes.as_ref(), None);

        Ok(())
    }

    /// Interprete the given bytes as an archived type.
    fn as_archive(bytes: &[u8]) -> Result<&Archived<Self>, ExpireError> {
        #[cfg(feature = "bytecheck")]
        {
            rkyv::access::<Archived<Self>, rkyv::rancor::BoxedError>(bytes)
                .map_err(From::from)
                .map_err(ExpireError::Validation)
        }

        #[cfg(not(feature = "bytecheck"))]
        unsafe {
            Ok(rkyv::access_unchecked::<Archived<Self>>(bytes))
        }
    }
}

/// Parse a slice into an [`Id<T>`].
pub(super) fn atoi<T>(bytes: &[u8]) -> Option<Id<T>> {
    bytes
        .iter()
        .try_fold(0_u64, |n, byte| {
            if !byte.is_ascii_digit() {
                return None;
            }

            n.checked_mul(10)?.checked_add(u64::from(*byte & 0xF))
        })
        .and_then(Id::new_checked)
}
