use tracing::{instrument, trace};
use randy_model::{
    gateway::presence::{Presence, UserOrId},
    id::{
        marker::{GuildMarker, UserMarker},
        Id,
    },
};

use crate::{
    cache::{
        meta::{atoi, IMetaKey},
        pipe::Pipe,
    },
    config::{CacheConfig, Cacheable, ICachedPresence, SerializeMany},
    error::{SerializeError, SerializeErrorKind},
    key::RedisKey,
    redis::Pipeline,
    util::BytesWrap,
    CacheResult, RedisCache,
};

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
            let key = RedisKey::Presence {
                guild: guild_id,
                user: user_id,
            };
            let presence = C::Presence::from_presence(presence);

            let bytes = presence
                .serialize_one()
                .map_err(|e| SerializeError::new(e, SerializeErrorKind::Presence))?;

            trace!(bytes = bytes.as_ref().len());

            pipe.set(key, bytes.as_ref(), C::Presence::expire());

            let key = RedisKey::GuildPresences { id: guild_id };
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
                    let key = RedisKey::Presence {
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
                .collect::<CacheResult<(Vec<(RedisKey, BytesWrap<_>)>, Vec<u64>)>>()?;

            if !presence_entries.is_empty() {
                pipe.mset(&presence_entries, C::Presence::expire());

                let key = RedisKey::GuildPresences { id: guild_id };
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
        let key = RedisKey::GuildPresences { id: self.guild };
        pipe.srem(key, self.user.get());
    }
}
