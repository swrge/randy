use crate::cache::impls::GuildEmojisKey;
use crate::cache::meta::atoi;
use crate::cache::meta::HasArchived;
use crate::cache::meta::IMeta;
use crate::cache::meta::IMetaKey;
use crate::config::Cacheable;
use crate::config::ICachedEmoji;
use crate::config::SerializeMany;
use crate::error::MetaError;
use crate::error::MetaErrorKind;
use crate::rkyv_util::id::IdRkyv;
use crate::{
    cache::pipe::Pipe,
    config::CacheConfig,
    error::{SerializeError, SerializeErrorKind},
    key::{name_id, RedisKey},
    redis::{Pipeline, RedisWrite, ToRedisArgs},
    util::BytesWrap,
    CacheResult, RedisCache,
};
use randy_model::{
    guild::Emoji,
    id::{
        marker::{EmojiMarker, GuildMarker},
        Id,
    },
};
use rkyv::api::high::to_bytes_in;
use rkyv::rancor::Source;
use rkyv::ser::writer::Buffer;
use rkyv::Archived;
use tracing::{instrument, trace};

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct EmojiKey {
    pub id: Id<EmojiMarker>,
}

impl RedisKey for EmojiKey {
    const PREFIX: &'static [u8] = b"EMOJI";
}

impl ToRedisArgs for EmojiKey {
    fn write_redis_args<W>(&self, out: &mut W)
    where
        W: ?Sized + RedisWrite,
    {
        out.write_arg(name_id(Self::PREFIX, self.id).as_ref());
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct EmojiMetaKey {
    pub emoji: Id<EmojiMarker>,
}

impl IMetaKey for EmojiMetaKey {
    fn parse<'a>(split: &mut impl Iterator<Item = &'a [u8]>) -> Option<Self> {
        split.next().and_then(atoi).map(|emoji| Self { emoji })
    }

    fn handle_expire(&self, pipe: &mut Pipeline) {
        let key = EmojisKey;
        pipe.srem(key, self.emoji.get()).ignore();
    }
}

impl HasArchived for EmojiMetaKey {
    type Meta = EmojiMeta;

    fn redis_key(&self) -> EmojiMetaKey {
        EmojiMetaKey { emoji: self.emoji }
    }

    fn handle_archived(&self, pipe: &mut Pipeline, archived: &Archived<Self::Meta>) {
        let key = GuildEmojisKey {
            id: archived.guild.into(),
        };
        pipe.srem(key, self.emoji.get());
    }
}

impl RedisKey for EmojiMetaKey {
    const PREFIX: &'static [u8] = b"EMOJI_META";
}

impl ToRedisArgs for EmojiMetaKey {
    fn write_redis_args<W>(&self, out: &mut W)
    where
        W: ?Sized + RedisWrite,
    {
        out.write_arg(name_id(Self::PREFIX, self.emoji).as_ref());
    }
}

#[derive(rkyv::Archive, rkyv::Serialize)]
pub(crate) struct EmojiMeta {
    #[rkyv(with = IdRkyv)]
    guild: Id<GuildMarker>,
}

impl IMeta<EmojiMetaKey> for EmojiMeta {
    type Bytes = [u8; 8];

    fn to_bytes<E: Source>(&self) -> Result<Self::Bytes, E> {
        let mut bytes = [0; 8];
        to_bytes_in(self, Buffer::from(&mut bytes))?;

        Ok(bytes)
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct EmojisKey;

impl RedisKey for EmojisKey {
    const PREFIX: &'static [u8] = b"EMOJIS";
}

impl ToRedisArgs for EmojisKey {
    fn write_redis_args<W>(&self, out: &mut W)
    where
        W: ?Sized + RedisWrite,
    {
        out.write_arg(Self::PREFIX);
    }
}

impl From<Id<EmojiMarker>> for EmojiKey {
    fn from(id: Id<EmojiMarker>) -> Self {
        Self { id }
    }
}

impl<C: CacheConfig> RedisCache<C> {
    #[instrument(level = "trace", skip_all)]
    pub(crate) fn store_emojis(
        &self,
        pipe: &mut Pipe<'_, C>,
        guild_id: Id<GuildMarker>,
        emojis: &[Emoji],
    ) -> CacheResult<()> {
        if !C::Emoji::WANTED {
            return Ok(());
        }

        let mut serializer = C::Emoji::serialize_many();

        let (emoji_entries, emoji_ids) = emojis
            .iter()
            .map(|emoji| {
                let id = emoji.id;
                let key = EmojiKey { id };
                let emoji = C::Emoji::from_emoji(emoji);

                let bytes = serializer
                    .serialize_next(&emoji)
                    .map_err(|e| SerializeError::new(e, SerializeErrorKind::Emoji))?;

                trace!(bytes = bytes.as_ref().len());

                Ok(((key, BytesWrap(bytes)), id.get()))
            })
            .collect::<CacheResult<(Vec<(EmojiKey, BytesWrap<_>)>, Vec<u64>)>>()?;

        if emoji_entries.is_empty() {
            return Ok(());
        }

        pipe.mset(&emoji_entries, C::Emoji::expire());

        let key = GuildEmojisKey { id: guild_id };
        pipe.sadd(key, emoji_ids.as_slice());

        let key = EmojisKey;
        pipe.sadd(key, emoji_ids);

        if C::Emoji::expire().is_some() {
            emojis
                .iter()
                .try_for_each(|emoji| {
                    let key = EmojiMetaKey { emoji: emoji.id };

                    EmojiMeta { guild: guild_id }.store(pipe, key)
                })
                .map_err(|e| MetaError::new(e, MetaErrorKind::Emoji))?;
        }

        Ok(())
    }
}
