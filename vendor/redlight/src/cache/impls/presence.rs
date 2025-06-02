use randy_model::{
    gateway::presence::{Presence, UserOrId},
    id::{
        marker::{GuildMarker, UserMarker},
        Id,
    },
};
use tracing::{instrument, trace};

use crate::{
    cache::{
        meta::{atoi, IMetaKey},
        pipe::Pipe,
    },
    config::{CacheConfig, Cacheable, ICachedPresence, SerializeMany},
    error::{SerializeError, SerializeErrorKind},
    key::{name_guild_id, RedisKey},
    redis::{Pipeline, RedisWrite, ToRedisArgs},
    util::BytesWrap,
    CacheResult, RedisCache,
};

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct PresenceKey {
    pub guild: Id<GuildMarker>,
    pub user: Id<UserMarker>,
}

impl RedisKey for PresenceKey {
    const PREFIX: &'static [u8] = b"PRESENCE";
}

impl ToRedisArgs for PresenceKey {
    fn write_redis_args<W>(&self, out: &mut W)
    where
        W: ?Sized + RedisWrite,
    {
        out.write_arg(name_guild_id(Self::PREFIX, self.guild, self.user).as_ref());
    }
}

#[derive(Debug)]
pub(crate) struct PresenceMetaKey {
    guild: Id<GuildMarker>,
    user: Id<UserMarker>,
}

impl IMetaKey for PresenceMetaKey {
    fn parse<'a>(split: &mut impl Iterator<Item = &'a [u8]>) -> Option<Self> {
        split
            .next()
            .and_then(atoi)
            .zip(split.next().and_then(atoi))
            .map(|(guild, user)| Self { guild, user })
    }

    fn handle_expire(&self, pipe: &mut Pipeline) {
        let key = crate::cache::impls::guild::GuildPresencesKey { id: self.guild };
        pipe.srem(key, self.user.get());
    }
}

impl<C: CacheConfig> RedisCache<C> {
    #[instrument(level = "trace", skip_all)]
    pub(crate) fn store_presence(
        &self,
        pipe: &mut Pipe<'_, C>,
        presence: &Presence,
    ) -> CacheResult<()> {
        if C::Presence::WANTED {
            let guild_id = presence.guild_id;
            let user_id = presence.user.id();
            let key = PresenceKey {
                guild: guild_id,
                user: user_id,
            };
            let presence = C::Presence::from_presence(presence);

            let bytes = presence
                .serialize_one()
                .map_err(|e| SerializeError::new(e, SerializeErrorKind::Presence))?;

            trace!(bytes = bytes.as_ref().len());

            pipe.set(key, bytes.as_ref(), C::Presence::expire());

            let key = crate::cache::impls::guild::GuildPresencesKey { id: guild_id };
            pipe.sadd(key, user_id.get());
        }

        if let UserOrId::User(ref user) = presence.user {
            self.store_user(pipe, user)?;
        }

        Ok(())
    }

    #[instrument(level = "trace", skip_all)]
    pub(crate) fn store_presences(
        &self,
        pipe: &mut Pipe<'_, C>,
        guild_id: Id<GuildMarker>,
        presences: &[Presence],
    ) -> CacheResult<()> {
        if C::Presence::WANTED {
            let mut serializer = C::Presence::serialize_many();

            let (presence_entries, user_ids) = presences
                .iter()
                .map(|presence| {
                    let guild_id = presence.guild_id;
                    let user_id = presence.user.id();
                    let key = PresenceKey {
                        guild: guild_id,
                        user: user_id,
                    };
                    let presence = C::Presence::from_presence(presence);

                    let bytes = serializer
                        .serialize_next(&presence)
                        .map_err(|e| SerializeError::new(e, SerializeErrorKind::Presence))?;

                    trace!(bytes = bytes.as_ref().len());

                    Ok(((key, BytesWrap(bytes)), user_id.get()))
                })
                .collect::<CacheResult<(Vec<(PresenceKey, BytesWrap<_>)>, Vec<u64>)>>()?;

            if !presence_entries.is_empty() {
                pipe.mset(&presence_entries, C::Presence::expire());

                let key = crate::cache::impls::guild::GuildPresencesKey { id: guild_id };
                pipe.sadd(key, user_ids.as_slice());
            }
        }

        let users = presences.iter().filter_map(|presence| match presence.user {
            UserOrId::User(ref user) => Some(user),
            UserOrId::UserId { .. } => None,
        });

        self.store_users(pipe, users)?;

        Ok(())
    }
}
