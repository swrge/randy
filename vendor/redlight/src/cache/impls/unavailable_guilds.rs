use crate::{
    key::RedisKey,
    redis::{RedisWrite, ToRedisArgs},
};

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct UnavailableGuildsKey;

impl RedisKey for UnavailableGuildsKey {
    const PREFIX: &'static [u8] = b"UNAVAILABLE_GUILDS";
}

impl ToRedisArgs for UnavailableGuildsKey {
    fn write_redis_args<W>(&self, out: &mut W)
    where
        W: ?Sized + RedisWrite,
    {
        out.write_arg(Self::PREFIX);
    }
}
