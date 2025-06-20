use randy_model::{
    id::{
        marker::{ChannelMarker, GuildMarker, UserMarker},
        Id,
    },
    voice::VoiceState,
};
use tracing::{instrument, trace};

use crate::{
    cache::{
        meta::{atoi, IMetaKey},
        pipe::Pipe,
    },
    config::{CacheConfig, Cacheable, ICachedVoiceState, SerializeMany},
    error::{CacheError, SerializeError, SerializeErrorKind},
    key::{name_guild_id, RedisKey},
    redis::{Pipeline, RedisWrite, ToRedisArgs},
    util::BytesWrap,
    CacheResult, RedisCache,
};

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct VoiceStateKey {
    pub guild: Id<GuildMarker>,
    pub user: Id<UserMarker>,
}

impl RedisKey for VoiceStateKey {
    const PREFIX: &'static [u8] = b"VOICE_STATE";
}

impl ToRedisArgs for VoiceStateKey {
    fn write_redis_args<W>(&self, out: &mut W)
    where
        W: ?Sized + RedisWrite,
    {
        out.write_arg(name_guild_id(Self::PREFIX, self.guild, self.user).as_ref());
    }
}

#[derive(Debug)]
pub(crate) struct VoiceStateMetaKey {
    guild: Id<GuildMarker>,
    user: Id<UserMarker>,
}

impl IMetaKey for VoiceStateMetaKey {
    fn parse<'a>(split: &mut impl Iterator<Item = &'a [u8]>) -> Option<Self> {
        split
            .next()
            .and_then(atoi)
            .zip(split.next().and_then(atoi))
            .map(|(guild, user)| Self { guild, user })
    }

    fn handle_expire(&self, pipe: &mut Pipeline) {
        let key = crate::cache::impls::guild::GuildVoiceStatesKey { id: self.guild };
        pipe.srem(key, self.user.get());
    }
}

impl<C: CacheConfig> RedisCache<C> {
    #[instrument(level = "trace", skip_all)]
    pub(crate) fn store_voice_state(
        &self,
        pipe: &mut Pipe<'_, C>,
        channel_id: Id<ChannelMarker>,
        guild_id: Id<GuildMarker>,
        voice_state: &VoiceState,
    ) -> CacheResult<()> {
        if C::VoiceState::WANTED {
            let user_id = voice_state.user_id;
            let key = VoiceStateKey {
                guild: guild_id,
                user: user_id,
            };
            let voice_state = C::VoiceState::from_voice_state(channel_id, guild_id, voice_state);

            let bytes = voice_state
                .serialize_one()
                .map_err(|e| SerializeError::new(e, SerializeErrorKind::VoiceState))?;

            trace!(bytes = bytes.as_ref().len());

            pipe.set(key, bytes.as_ref(), C::VoiceState::expire());

            let key = crate::cache::impls::guild::GuildVoiceStatesKey { id: guild_id };
            pipe.sadd(key, user_id.get());
        }

        if let Some(ref member) = voice_state.member {
            self.store_member(pipe, guild_id, member)?;
        }

        Ok(())
    }

    #[instrument(level = "trace", skip_all)]
    pub(crate) fn store_voice_states(
        &self,
        pipe: &mut Pipe<'_, C>,
        guild_id: Id<GuildMarker>,
        voice_states: &[VoiceState],
    ) -> CacheResult<()> {
        if !C::VoiceState::WANTED {
            return Ok(());
        }

        let mut serializer = C::VoiceState::serialize_many();

        let (voice_states, user_ids) = voice_states
            .iter()
            .filter_map(|voice_state| {
                let channel_id = voice_state.channel_id?;

                let user_id = voice_state.user_id;
                let key = VoiceStateKey {
                    guild: guild_id,
                    user: user_id,
                };
                let voice_state =
                    C::VoiceState::from_voice_state(channel_id, guild_id, voice_state);

                let res = serializer
                    .serialize_next(&voice_state)
                    .map(|bytes| {
                        trace!(bytes = bytes.as_ref().len());

                        ((key, BytesWrap(bytes)), user_id.get())
                    })
                    .map_err(|e| {
                        CacheError::Serialization(SerializeError::new(
                            e,
                            SerializeErrorKind::VoiceState,
                        ))
                    });

                Some(res)
            })
            .collect::<CacheResult<(Vec<(VoiceStateKey, BytesWrap<_>)>, Vec<u64>)>>()?;

        if voice_states.is_empty() {
            return Ok(());
        }

        pipe.mset(&voice_states, C::VoiceState::expire());

        let key = crate::cache::impls::guild::GuildVoiceStatesKey { id: guild_id };
        pipe.sadd(key, user_ids.as_slice());

        Ok(())
    }

    pub(crate) fn delete_voice_state(
        &self,
        pipe: &mut Pipe<'_, C>,
        guild_id: Id<GuildMarker>,
        user_id: Id<UserMarker>,
    ) {
        if !C::VoiceState::WANTED {
            return;
        }

        let key = VoiceStateKey {
            guild: guild_id,
            user: user_id,
        };
        pipe.del(key);

        let key = crate::cache::impls::guild::GuildVoiceStatesKey { id: guild_id };
        pipe.srem(key, user_id.get());
    }
}
