use randy_model::user::CurrentUser;
use tracing::{instrument, trace};

use crate::{
    cache::pipe::Pipe,
    config::{CacheConfig, Cacheable, ICachedCurrentUser},
    error::{SerializeError, SerializeErrorKind},
    key::RedisKey,
    redis::{RedisWrite, ToRedisArgs},
    CacheResult, RedisCache,
};

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct CurrentUserKey;

impl RedisKey for CurrentUserKey {
    const PREFIX: &'static [u8] = b"CURRENT_USER";
}

impl ToRedisArgs for CurrentUserKey {
    fn write_redis_args<W>(&self, out: &mut W)
    where
        W: ?Sized + RedisWrite,
    {
        out.write_arg(Self::PREFIX);
    }
}

impl<C: CacheConfig> RedisCache<C> {
    #[instrument(level = "trace", skip_all)]
    pub(crate) fn store_current_user(
        &self,
        pipe: &mut Pipe<'_, C>,
        current_user: &CurrentUser,
    ) -> CacheResult<()> {
        if !C::CurrentUser::WANTED {
            return Ok(());
        }

        let key = CurrentUserKey;
        let current_user = C::CurrentUser::from_current_user(current_user);

        let bytes = current_user
            .serialize_one()
            .map_err(|e| SerializeError::new(e, SerializeErrorKind::CurrentUser))?;

        trace!(bytes = bytes.as_ref().len());

        pipe.set(key, bytes.as_ref(), C::CurrentUser::expire());

        Ok(())
    }
}
