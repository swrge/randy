use randy_model::{
    channel::message::Sticker,
    id::{
        marker::{GuildMarker, StickerMarker},
        Id,
    },
};
use rkyv::{api::high::to_bytes_in, rancor::Source, ser::writer::Buffer, Archived};
use tracing::{instrument, trace};

use crate::{
    cache::{
        meta::{atoi, HasArchived, IMeta, IMetaKey},
        pipe::Pipe,
    },
    config::{CacheConfig, Cacheable, ICachedSticker, SerializeMany},
    error::{MetaError, MetaErrorKind, SerializeError, SerializeErrorKind},
    key::{name_id, RedisKey},
    redis::{Pipeline, RedisWrite, ToRedisArgs},
    rkyv_util::id::IdRkyv,
    util::BytesWrap,
    CacheResult, RedisCache,
};

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct StickerKey {
    pub id: Id<StickerMarker>,
}

impl RedisKey for StickerKey {
    const PREFIX: &'static [u8] = b"STICKER";
}

impl ToRedisArgs for StickerKey {
    fn write_redis_args<W>(&self, out: &mut W)
    where
        W: ?Sized + RedisWrite,
    {
        out.write_arg(name_id(Self::PREFIX, self.id).as_ref());
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct StickerMetaKey {
    pub id: Id<StickerMarker>,
}

impl RedisKey for StickerMetaKey {
    const PREFIX: &'static [u8] = b"STICKER_META";
}

impl ToRedisArgs for StickerMetaKey {
    fn write_redis_args<W>(&self, out: &mut W)
    where
        W: ?Sized + RedisWrite,
    {
        out.write_arg(name_id(Self::PREFIX, self.id).as_ref());
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct StickersKey;

impl RedisKey for StickersKey {
    const PREFIX: &'static [u8] = b"STICKERS";
}

impl ToRedisArgs for StickersKey {
    fn write_redis_args<W>(&self, out: &mut W)
    where
        W: ?Sized + RedisWrite,
    {
        out.write_arg(Self::PREFIX);
    }
}

impl From<Id<StickerMarker>> for StickerKey {
    fn from(id: Id<StickerMarker>) -> Self {
        Self { id }
    }
}

impl<C: CacheConfig> RedisCache<C> {
    #[instrument(level = "trace", skip_all)]
    pub(crate) fn store_stickers(
        &self,
        pipe: &mut Pipe<'_, C>,
        guild_id: Id<GuildMarker>,
        stickers: &[Sticker],
    ) -> CacheResult<()> {
        if !C::Sticker::WANTED {
            return Ok(());
        }

        let mut serializer = C::Sticker::serialize_many();

        let (sticker_entries, sticker_ids) = stickers
            .iter()
            .map(|sticker| {
                let id = sticker.id;
                let key = StickerKey { id };
                let sticker = C::Sticker::from_sticker(sticker);

                let bytes = serializer
                    .serialize_next(&sticker)
                    .map_err(|e| SerializeError::new(e, SerializeErrorKind::Sticker))?;

                trace!(bytes = bytes.as_ref().len());

                Ok(((key, BytesWrap(bytes)), id.get()))
            })
            .collect::<CacheResult<(Vec<(StickerKey, BytesWrap<_>)>, Vec<u64>)>>()?;

        if sticker_entries.is_empty() {
            return Ok(());
        }

        pipe.mset(&sticker_entries, C::Sticker::expire());

        let key = crate::cache::impls::guild::GuildStickersKey { id: guild_id };
        pipe.sadd(key, sticker_ids.as_slice());

        let key = StickersKey;
        pipe.sadd(key, sticker_ids);

        if C::Sticker::expire().is_some() {
            stickers
                .iter()
                .try_for_each(|sticker| {
                    let key = StickerMetaKey { id: sticker.id };

                    StickerMeta { guild: guild_id }.store(pipe, key)
                })
                .map_err(|e| MetaError::new(e, MetaErrorKind::Sticker))?;
        }

        Ok(())
    }

    pub(crate) fn delete_sticker(
        &self,
        pipe: &mut Pipe<'_, C>,
        guild_id: Id<GuildMarker>,
        sticker_id: Id<StickerMarker>,
    ) {
        if !C::Sticker::WANTED {
            return;
        }

        let key = StickerKey { id: sticker_id };
        pipe.del(key);

        let key = crate::cache::impls::guild::GuildStickersKey { id: guild_id };
        pipe.srem(key, sticker_id.get());

        let key = StickersKey;
        pipe.srem(key, sticker_id.get());

        if C::Sticker::expire().is_some() {
            let key = StickerMetaKey { id: sticker_id };
            pipe.del(key);
        }
    }
}

impl IMetaKey for StickerMetaKey {
    fn parse<'a>(split: &mut impl Iterator<Item = &'a [u8]>) -> Option<Self> {
        split
            .next()
            .and_then(atoi)
            .map(|sticker| Self { id: sticker })
    }

    fn handle_expire(&self, pipe: &mut Pipeline) {
        let key = StickersKey;
        pipe.srem(key, self.id.get()).ignore();
    }
}

impl HasArchived for StickerMetaKey {
    type Meta = StickerMeta;

    fn redis_key(&self) -> impl RedisKey {
        StickerMetaKey { id: self.id }
    }

    fn handle_archived(&self, pipe: &mut Pipeline, archived: &Archived<Self::Meta>) {
        let key = crate::cache::impls::guild::GuildStickersKey {
            id: archived.guild.into(),
        };
        pipe.srem(key, self.id.get());
    }
}

#[derive(rkyv::Archive, rkyv::Serialize)]
pub(crate) struct StickerMeta {
    #[rkyv(with = IdRkyv)]
    guild: Id<GuildMarker>,
}

impl IMeta<StickerMetaKey> for StickerMeta {
    type Bytes = [u8; 8];

    fn to_bytes<E: Source>(&self) -> Result<Self::Bytes, E> {
        let mut bytes = [0; 8];
        to_bytes_in(self, Buffer::from(&mut bytes))?;

        Ok(bytes)
    }
}
