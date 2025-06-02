use std::borrow::Cow;

use crate::{
    cache::{
        impls::GuildChannelsKey,
        meta::{atoi, HasArchived, IMeta, IMetaKey},
        pipe::Pipe,
    },
    config::{CacheConfig, Cacheable, ICachedChannel, SerializeMany},
    error::{
        MetaError, MetaErrorKind, SerializeError, SerializeErrorKind, UpdateError, UpdateErrorKind,
    },
    key::{name_id, RedisKey},
    redis::{Pipeline, RedisWrite, ToRedisArgs},
    rkyv_util::id::IdRkyvMap,
    util::BytesWrap,
    CacheResult, RedisCache,
};
use randy_model::{
    channel::Channel,
    gateway::payload::incoming::ChannelPinsUpdate,
    id::{
        marker::{ChannelMarker, GuildMarker},
        Id,
    },
};
use rkyv::{
    api::high::to_bytes_in,
    rancor::{self, Source},
    ser::writer::Buffer,
    Archived,
};
use tracing::{instrument, trace};

impl<C: CacheConfig> RedisCache<C> {
    #[instrument(level = "trace", skip_all)]
    pub(crate) fn store_channel(
        &self,
        pipe: &mut Pipe<'_, C>,
        channel: &Channel,
    ) -> CacheResult<()> {
        if C::Channel::WANTED {
            let guild_id = channel.guild_id;
            let channel_id = channel.id;
            let key = ChannelKey { id: channel_id };
            let channel = C::Channel::from_channel(channel);

            let bytes = channel
                .serialize_one()
                .map_err(|e| SerializeError::new(e, SerializeErrorKind::Channel))?;

            trace!(bytes = bytes.as_ref().len());

            pipe.set(key, bytes.as_ref(), C::Channel::expire());

            if C::Channel::expire().is_some() {
                let key = ChannelMetaKey {
                    channel: channel_id,
                };

                ChannelMeta { guild: guild_id }
                    .store(pipe, key)
                    .map_err(|e| MetaError::new(e, MetaErrorKind::Channel))?;
            }

            if let Some(guild_id) = guild_id {
                let key = GuildChannelsKey { id: guild_id };
                pipe.sadd(key, channel_id.get());
            }

            let key = ChannelsKey;
            pipe.sadd(key, channel_id.get());
        }

        if let Some(ref member) = channel.member {
            if let (Some(guild_id), Some(member)) = (channel.guild_id, &member.member) {
                self.store_member(pipe, guild_id, member)?;
            }

            if let Some(ref presence) = member.presence {
                self.store_presence(pipe, presence)?;
            }
        }

        if let Some(ref users) = channel.recipients {
            self.store_users(pipe, users)?;
        }

        Ok(())
    }

    #[instrument(level = "trace", skip_all)]
    pub(crate) async fn store_channel_pins_update(
        &self,
        pipe: &mut Pipe<'_, C>,
        update: &ChannelPinsUpdate,
    ) -> CacheResult<()> {
        if !C::Channel::WANTED {
            return Ok(());
        }

        let Some(update_fn) = C::Channel::on_pins_update() else {
            return Ok(());
        };

        let key = ChannelKey {
            id: update.channel_id,
        };

        let Some(mut channel) = pipe.get::<Archived<C::Channel<'static>>>(key).await? else {
            return Ok(());
        };

        update_fn(&mut channel, update)
            .map_err(|e| UpdateError::new(e, UpdateErrorKind::ChannelPins))?;

        let key = ChannelKey {
            id: update.channel_id,
        };

        let bytes = channel.into_bytes();
        trace!(bytes = bytes.as_ref().len());
        pipe.set(key, &bytes, C::Channel::expire());

        if C::Channel::expire().is_some() {
            let key = ChannelMetaKey {
                channel: update.channel_id,
            };

            let meta = ChannelMeta {
                guild: update.guild_id,
            };

            meta.store(pipe, key)
                .map_err(|e| MetaError::new(e, MetaErrorKind::Channel))?;
        }

        Ok(())
    }

    #[instrument(level = "trace", skip_all)]
    pub(crate) fn store_channels(
        &self,
        pipe: &mut Pipe<'_, C>,
        guild_id: Id<GuildMarker>,
        channels: &[Channel],
    ) -> CacheResult<()> {
        if C::Channel::WANTED {
            let mut serializer = C::Channel::serialize_many();

            let (channel_entries, channel_ids) = channels
                .iter()
                .map(|channel| {
                    let id = channel.id;
                    let key = ChannelKey { id };
                    let channel = C::Channel::from_channel(channel);

                    let bytes = serializer
                        .serialize_next(&channel)
                        .map_err(|e| SerializeError::new(e, SerializeErrorKind::Channel))?;

                    trace!(bytes = bytes.as_ref().len());

                    Ok(((key, BytesWrap(bytes)), id.get()))
                })
                .collect::<CacheResult<(Vec<(ChannelKey, BytesWrap<_>)>, Vec<u64>)>>()?;

            if !channel_entries.is_empty() {
                pipe.mset(&channel_entries, C::Channel::expire());

                let key = GuildChannelsKey { id: guild_id };
                pipe.sadd(key, channel_ids.as_slice());

                let key = ChannelsKey;
                pipe.sadd(key, channel_ids);

                if C::Channel::expire().is_some() {
                    channels
                        .iter()
                        .try_for_each(|channel| {
                            let key = ChannelMetaKey {
                                channel: channel.id,
                            };

                            let meta = ChannelMeta {
                                guild: channel.guild_id,
                            };

                            meta.store(pipe, key)
                        })
                        .map_err(|e| MetaError::new(e, MetaErrorKind::Channel))?;
                }
            }
        }

        let users = channels
            .iter()
            .filter_map(|channel| channel.recipients.as_ref())
            .flatten();

        self.store_users(pipe, users)?;

        Ok(())
    }

    pub(crate) fn delete_channel(
        &self,
        pipe: &mut Pipe<'_, C>,
        guild_id: Option<Id<GuildMarker>>,
        channel_id: Id<ChannelMarker>,
    ) {
        if !C::Channel::WANTED {
            return;
        }

        let key = ChannelKey { id: channel_id };
        pipe.del(key);

        if let Some(guild_id) = guild_id {
            let key = GuildChannelsKey { id: guild_id };
            pipe.srem(key, channel_id.get());
        }

        let key = ChannelsKey;
        pipe.srem(key, channel_id.get());

        if C::Channel::expire().is_some() {
            pipe.del(ChannelMetaKey {
                channel: channel_id,
            });
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct ChannelKey {
    pub id: Id<ChannelMarker>,
}

impl RedisKey for ChannelKey {
    const PREFIX: &'static [u8] = b"CHANNEL";
}

impl ToRedisArgs for ChannelKey {
    fn write_redis_args<W>(&self, out: &mut W)
    where
        W: ?Sized + RedisWrite,
    {
        out.write_arg(name_id(Self::PREFIX, self.id).as_ref());
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct ChannelMessagesKey {
    pub channel: Id<ChannelMarker>,
}

impl RedisKey for ChannelMessagesKey {
    const PREFIX: &'static [u8] = b"CHANNEL_MESSAGES_META";
}

impl ToRedisArgs for ChannelMessagesKey {
    fn write_redis_args<W>(&self, out: &mut W)
    where
        W: ?Sized + RedisWrite,
    {
        out.write_arg(name_id(Self::PREFIX, self.channel).as_ref());
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct ChannelMetaKey {
    pub channel: Id<ChannelMarker>,
}

impl RedisKey for ChannelMetaKey {
    const PREFIX: &'static [u8] = b"CHANNEL_META";
}

impl ToRedisArgs for ChannelMetaKey {
    fn write_redis_args<W>(&self, out: &mut W)
    where
        W: ?Sized + RedisWrite,
    {
        out.write_arg(name_id(Self::PREFIX, self.channel).as_ref());
    }
}

impl IMetaKey for ChannelMetaKey {
    fn parse<'a>(split: &mut impl Iterator<Item = &'a [u8]>) -> Option<Self> {
        split.next().and_then(atoi).map(|channel_id| Self {
            channel: channel_id,
        })
    }

    fn handle_expire(&self, pipe: &mut Pipeline) {
        let key = ChannelsKey;
        pipe.srem(key, self.channel.get()).ignore();
    }
}

impl HasArchived for ChannelMetaKey {
    type Meta = ChannelMeta;

    fn redis_key(&self) -> ChannelMetaKey {
        ChannelMetaKey {
            channel: self.channel,
        }
    }

    fn handle_archived(&self, pipe: &mut Pipeline, archived: &Archived<Self::Meta>) {
        if let Some(archived) = archived.guild.as_ref() {
            let guild = Id::from(*archived);
            let key = GuildChannelsKey { id: guild };
            pipe.srem(key, self.channel.get());
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct ChannelsKey;

impl RedisKey for ChannelsKey {
    const PREFIX: &'static [u8] = b"CHANNELS";
}

impl ToRedisArgs for ChannelsKey {
    fn write_redis_args<W>(&self, out: &mut W)
    where
        W: ?Sized + RedisWrite,
    {
        out.write_arg(Self::PREFIX);
    }
}

impl From<Id<ChannelMarker>> for ChannelKey {
    fn from(id: Id<ChannelMarker>) -> Self {
        Self { id }
    }
}

#[derive(rkyv::Archive, rkyv::Serialize)]
pub(crate) struct ChannelMeta {
    #[rkyv(with = IdRkyvMap)]
    guild: Option<Id<GuildMarker>>,
}

impl IMeta<ChannelMetaKey> for ChannelMeta {
    type Bytes = [u8; 8];

    fn to_bytes<E: Source>(&self) -> Result<Self::Bytes, E> {
        let mut bytes = [0; 8];
        to_bytes_in(self, Buffer::from(&mut bytes))?;

        Ok(bytes)
    }
}
