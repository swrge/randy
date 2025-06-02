use randy_model::{
    gateway::payload::incoming::invite_create::PartialUser,
    id::{
        marker::{GuildMarker, UserMarker},
        Id,
    },
    user::User,
};
use rkyv::Archived;
use tracing::{instrument, trace};

use crate::{
    cache::{
        meta::{atoi, IMetaKey},
        pipe::Pipe,
    },
    config::{CacheConfig, Cacheable, ICachedUser, SerializeMany},
    error::{SerializeError, SerializeErrorKind, UpdateError, UpdateErrorKind},
    key::{name_id, RedisKey},
    redis::{Pipeline, RedisWrite, ToRedisArgs},
    util::BytesWrap,
    CacheResult, RedisCache,
};

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct UserKey {
    pub id: Id<UserMarker>,
}

impl RedisKey for UserKey {
    const PREFIX: &'static [u8] = b"USER";
}

impl ToRedisArgs for UserKey {
    fn write_redis_args<W>(&self, out: &mut W)
    where
        W: ?Sized + RedisWrite,
    {
        out.write_arg(name_id(Self::PREFIX, self.id).as_ref());
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct UserGuildsKey {
    pub id: Id<UserMarker>,
}

impl RedisKey for UserGuildsKey {
    const PREFIX: &'static [u8] = b"USER_GUILDS";
}

impl ToRedisArgs for UserGuildsKey {
    fn write_redis_args<W>(&self, out: &mut W)
    where
        W: ?Sized + RedisWrite,
    {
        out.write_arg(name_id(Self::PREFIX, self.id).as_ref());
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct UsersKey;

impl RedisKey for UsersKey {
    const PREFIX: &'static [u8] = b"USERS";
}

impl ToRedisArgs for UsersKey {
    fn write_redis_args<W>(&self, out: &mut W)
    where
        W: ?Sized + RedisWrite,
    {
        out.write_arg(Self::PREFIX);
    }
}

impl From<Id<UserMarker>> for UserKey {
    fn from(id: Id<UserMarker>) -> Self {
        Self { id }
    }
}

impl<C: CacheConfig> RedisCache<C> {
    #[instrument(level = "trace", skip_all)]
    pub(crate) fn store_user(&self, pipe: &mut Pipe<'_, C>, user: &User) -> CacheResult<()> {
        if !C::User::WANTED {
            return Ok(());
        }

        let id = user.id;
        let key = UserKey { id };
        let user = C::User::from_user(user);

        let bytes = user
            .serialize_one()
            .map_err(|e| SerializeError::new(e, SerializeErrorKind::User))?;

        trace!(bytes = bytes.as_ref().len());

        pipe.set(key, bytes.as_ref(), C::User::expire());

        let key = UsersKey;
        pipe.sadd(key, id.get());

        Ok(())
    }

    #[instrument(level = "trace", skip_all)]
    pub(crate) fn store_users<'a, I>(&self, pipe: &mut Pipe<'_, C>, users: I) -> CacheResult<()>
    where
        I: IntoIterator<Item = &'a User>,
    {
        if !C::User::WANTED {
            return Ok(());
        }

        let mut serializer = C::User::serialize_many();

        let (users, user_ids) = users
            .into_iter()
            .map(|user| {
                let id = user.id;
                let key = UserKey { id };
                let user = C::User::from_user(user);

                let bytes = serializer
                    .serialize_next(&user)
                    .map_err(|e| SerializeError::new(e, SerializeErrorKind::User))?;

                trace!(bytes = bytes.as_ref().len());

                Ok(((key, BytesWrap(bytes)), id.get()))
            })
            .collect::<CacheResult<(Vec<(UserKey, BytesWrap<_>)>, Vec<u64>)>>()?;

        if users.is_empty() {
            return Ok(());
        }

        pipe.mset(&users, C::User::expire());

        let key = UsersKey;
        pipe.sadd(key, user_ids);

        Ok(())
    }

    pub(crate) async fn store_partial_user(
        &self,
        pipe: &mut Pipe<'_, C>,
        partial_user: &PartialUser,
    ) -> CacheResult<()> {
        if !C::User::WANTED {
            return Ok(());
        }

        let id = partial_user.id;

        let key = UsersKey;
        pipe.sadd(key, id.get());

        let Some(update_fn) = C::User::update_via_partial() else {
            return Ok(());
        };

        let key = UserKey { id };

        let Some(mut user) = pipe.get::<Archived<C::User<'static>>>(key).await? else {
            return Ok(());
        };

        update_fn(&mut user, partial_user)
            .map_err(|e| UpdateError::new(e, UpdateErrorKind::PartialUser))?;

        let key = UserKey { id };
        let bytes = user.into_bytes();
        pipe.set(key, &bytes, C::Guild::expire());

        Ok(())
    }

    pub(crate) async fn delete_user(
        &self,
        pipe: &mut Pipe<'_, C>,
        user_id: Id<UserMarker>,
        guild_id: Id<GuildMarker>,
    ) -> CacheResult<()> {
        if !C::User::WANTED {
            return Ok(());
        }

        debug_assert!(pipe.is_empty());

        let key = UserGuildsKey { id: user_id };
        pipe.srem(key, guild_id.get());

        let key = UserGuildsKey { id: user_id };
        pipe.scard(key);

        let common_guild_count: usize = pipe.query().await?;

        if common_guild_count == 0 {
            let key = UserKey { id: user_id };
            pipe.del(key);

            let key = UsersKey;
            pipe.srem(key, user_id.get());
        }

        Ok(())
    }
}

#[derive(Debug)]
pub(crate) struct UserMetaKey {
    pub user: Id<UserMarker>,
}

impl IMetaKey for UserMetaKey {
    fn parse<'a>(split: &mut impl Iterator<Item = &'a [u8]>) -> Option<Self> {
        split.next().and_then(atoi).map(|user| Self { user })
    }

    fn handle_expire(&self, pipe: &mut Pipeline) {
        let key = UsersKey;
        pipe.srem(key, self.user.get()).ignore();

        let key = UserGuildsKey { id: self.user };
        pipe.del(key).ignore();
    }
}

impl UserMetaKey {
    pub(crate) const fn new(user: Id<UserMarker>) -> Self {
        Self { user }
    }
}
