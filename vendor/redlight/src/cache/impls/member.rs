use randy_model::{
    gateway::payload::incoming::MemberUpdate,
    guild::{Member, PartialMember},
    id::{
        marker::{GuildMarker, UserMarker},
        Id,
    },
};
use rkyv::Archived;
use tracing::{instrument, trace};

use crate::{
    cache::{
        impls::{
            guild::GuildMembersKey,
            user::{UserGuildsKey, UserMetaKey},
        },
        meta::{atoi, IMetaKey},
        pipe::Pipe,
    },
    config::{CacheConfig, Cacheable, ICachedMember, SerializeMany},
    error::{ExpireError, SerializeError, SerializeErrorKind, UpdateError, UpdateErrorKind},
    key::{name_guild_id, RedisKey},
    redis::{DedicatedConnection, Pipeline, RedisWrite, ToRedisArgs},
    util::BytesWrap,
    CacheResult, RedisCache,
};

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct MemberKey {
    pub guild: Id<GuildMarker>,
    pub user: Id<UserMarker>,
}

impl RedisKey for MemberKey {
    const PREFIX: &'static [u8] = b"MEMBER";
}

impl ToRedisArgs for MemberKey {
    fn write_redis_args<W>(&self, out: &mut W)
    where
        W: ?Sized + RedisWrite,
    {
        out.write_arg(name_guild_id(Self::PREFIX, self.guild, self.user).as_ref());
    }
}

impl From<(Id<GuildMarker>, Id<UserMarker>)> for MemberKey {
    fn from(ids: (Id<GuildMarker>, Id<UserMarker>)) -> Self {
        Self {
            guild: ids.0,
            user: ids.1,
        }
    }
}

#[derive(Debug)]
pub(crate) struct MemberMetaKey {
    guild: Id<GuildMarker>,
    user: Id<UserMarker>,
}

impl IMetaKey for MemberMetaKey {
    fn parse<'a>(split: &mut impl Iterator<Item = &'a [u8]>) -> Option<Self> {
        split
            .next()
            .and_then(atoi)
            .zip(split.next().and_then(atoi))
            .map(|(guild, user)| Self { guild, user })
    }

    fn handle_expire(&self, pipe: &mut Pipeline) {
        let key = GuildMembersKey { id: self.guild };
        pipe.srem(key, self.user.get()).ignore();
    }
}

impl MemberMetaKey {
    pub(crate) async fn async_handle_expire(
        &self,
        pipe: &mut Pipeline,
        conn: &mut DedicatedConnection,
    ) -> Result<(), ExpireError> {
        debug_assert_eq!(pipe.cmd_iter().count(), 0);

        let key = UserGuildsKey { id: self.user };

        let common_guild_count: usize = pipe
            .scard(key)
            .query_async(conn)
            .await
            .map_err(ExpireError::Pipe)?;

        pipe.clear();

        if common_guild_count == 1 {
            UserMetaKey::new(self.user).handle_expire(pipe);
        } else {
            let key = UserGuildsKey { id: self.user };
            pipe.srem(key, self.guild.get()).ignore();
        }

        Ok(())
    }
}

impl<C: CacheConfig> RedisCache<C> {
    #[instrument(level = "trace", skip_all)]
    pub(crate) fn store_member(
        &self,
        pipe: &mut Pipe<'_, C>,
        guild_id: Id<GuildMarker>,
        member: &Member,
    ) -> CacheResult<()> {
        if C::Member::WANTED {
            let user_id = member.user.id;
            let key = MemberKey {
                guild: guild_id,
                user: user_id,
            };
            let member = C::Member::from_member(guild_id, member);

            let bytes = member
                .serialize_one()
                .map_err(|e| SerializeError::new(e, SerializeErrorKind::Member))?;

            trace!(bytes = bytes.as_ref().len());

            pipe.set(key, bytes.as_ref(), C::Member::expire());

            let key = GuildMembersKey { id: guild_id };
            pipe.sadd(key, user_id.get());
        }

        if C::User::WANTED {
            let key = UserGuildsKey { id: member.user.id };
            pipe.sadd(key, guild_id.get());
        }

        self.store_user(pipe, &member.user)?;

        Ok(())
    }

    #[instrument(level = "trace", skip_all)]
    pub(crate) async fn store_member_update(
        &self,
        pipe: &mut Pipe<'_, C>,
        update: &MemberUpdate,
    ) -> CacheResult<()> {
        self.store_user(pipe, &update.user)?;

        let user_id = update.user.id;

        if C::User::WANTED {
            let key = UserGuildsKey { id: user_id };
            pipe.sadd(key, update.guild_id.get());
        }

        if !C::Member::WANTED {
            return Ok(());
        }

        let key = GuildMembersKey {
            id: update.guild_id,
        };

        pipe.sadd(key, user_id.get());

        let Some(update_fn) = C::Member::on_member_update() else {
            return Ok(());
        };

        let key = MemberKey {
            guild: update.guild_id,
            user: user_id,
        };

        let Some(mut member) = pipe.get::<Archived<C::Member<'static>>>(key).await? else {
            return Ok(());
        };

        update_fn(&mut member, update).map_err(|e| UpdateError::new(e, UpdateErrorKind::Member))?;

        let key = MemberKey {
            guild: update.guild_id,
            user: user_id,
        };

        let bytes = member.into_bytes();
        trace!(bytes = bytes.as_ref().len());
        pipe.set(key, &bytes, C::Member::expire());

        Ok(())
    }

    #[instrument(level = "trace", skip_all)]
    pub(crate) fn store_members(
        &self,
        pipe: &mut Pipe<'_, C>,
        guild_id: Id<GuildMarker>,
        members: &[Member],
    ) -> CacheResult<()> {
        if C::Member::WANTED {
            let mut serializer = C::Member::serialize_many();

            let (member_tuples, user_ids) = members
                .iter()
                .map(|member| {
                    let user_id = member.user.id;
                    let key = MemberKey {
                        guild: guild_id,
                        user: user_id,
                    };
                    let member = C::Member::from_member(guild_id, member);

                    let bytes = serializer
                        .serialize_next(&member)
                        .map_err(|e| SerializeError::new(e, SerializeErrorKind::Member))?;

                    trace!(bytes = bytes.as_ref().len());

                    Ok(((key, BytesWrap(bytes)), user_id.get()))
                })
                .collect::<CacheResult<(Vec<(MemberKey, BytesWrap<_>)>, Vec<u64>)>>()?;

            if !member_tuples.is_empty() {
                pipe.mset(&member_tuples, C::Member::expire());

                let key = GuildMembersKey { id: guild_id };
                pipe.sadd(key, user_ids.as_slice());

                if C::User::WANTED {
                    for member in members {
                        let key = UserGuildsKey { id: member.user.id };
                        pipe.sadd(key, guild_id.get());
                    }
                }
            }
        } else if C::User::WANTED {
            for member in members {
                let key = UserGuildsKey { id: member.user.id };
                pipe.sadd(key, guild_id.get());
            }
        }

        let users = members.iter().map(|member| &member.user);
        self.store_users(pipe, users)?;

        Ok(())
    }

    #[instrument(level = "trace", skip_all)]
    pub(crate) async fn store_partial_member(
        &self,
        pipe: &mut Pipe<'_, C>,
        guild_id: Id<GuildMarker>,
        partial_member: &PartialMember,
    ) -> CacheResult<()> {
        if let Some(ref user) = partial_member.user {
            self.store_user(pipe, user)?;
        }

        let Some(ref user) = partial_member.user else {
            return Ok(());
        };

        if C::User::WANTED {
            let key = UserGuildsKey { id: user.id };
            pipe.sadd(key, guild_id.get());
        }

        if !C::Member::WANTED {
            return Ok(());
        }

        let key = GuildMembersKey { id: guild_id };
        pipe.sadd(key, user.id.get());

        let Some(update_fn) = C::Member::update_via_partial() else {
            return Ok(());
        };

        let key = MemberKey {
            guild: guild_id,
            user: user.id,
        };

        let Some(mut member) = pipe.get::<Archived<C::Member<'static>>>(key).await? else {
            return Ok(());
        };

        update_fn(&mut member, partial_member)
            .map_err(|e| UpdateError::new(e, UpdateErrorKind::PartialMember))?;

        let key = MemberKey {
            guild: guild_id,
            user: user.id,
        };

        let bytes = member.into_bytes();
        trace!(bytes = bytes.as_ref().len());
        pipe.set(key, &bytes, C::Member::expire());

        Ok(())
    }

    pub(crate) async fn delete_member(
        &self,
        pipe: &mut Pipe<'_, C>,
        guild_id: Id<GuildMarker>,
        user_id: Id<UserMarker>,
    ) -> CacheResult<()> {
        self.delete_user(pipe, user_id, guild_id).await?;

        if !C::Member::WANTED {
            return Ok(());
        }

        let key = MemberKey {
            guild: guild_id,
            user: user_id,
        };
        pipe.del(key);

        let key = GuildMembersKey { id: guild_id };
        pipe.srem(key, user_id.get());

        Ok(())
    }
}
