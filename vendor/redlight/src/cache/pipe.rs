use std::time::Duration;

use rkyv::util::AlignedVec;
use tracing::{instrument, trace};

use crate::{
    config::{CacheConfig, CheckedArchived},
    key::RedisKey,
    redis::{Cmd, ConnectionState, FromRedisValue, Pipeline, ToRedisArgs},
    util::BytesWrap,
    CacheResult, CachedArchive, RedisCache,
};

pub struct Pipe<'c, C> {
    conn: ConnectionState<'c, C>,
    pipe: Pipeline,
}

impl<'c, C> Pipe<'c, C> {
    pub fn new(cache: &'c RedisCache<C>) -> Self {
        Self {
            conn: ConnectionState::new(cache),
            pipe: Pipeline::new(),
        }
    }

    pub fn len(&self) -> usize {
        self.pipe.cmd_iter().count()
    }

    pub async fn query<T: FromRedisValue>(&mut self) -> CacheResult<T> {
        trace!(piped = self.len());

        let conn = self.conn.get().await?;
        let res = self.pipe.query_async(conn).await?;
        self.pipe.clear();

        Ok(res)
    }

    pub fn del(&mut self, key: impl ToRedisArgs) {
        self.pipe.del(key).ignore();
    }

    pub fn is_empty(&self) -> bool {
        self.pipe.cmd_iter().next().is_none()
    }

    pub fn mset<V: ToRedisArgs>(&mut self, items: &[(impl RedisKey, V)], expire: Option<Duration>) {
        self.pipe.mset(items).ignore();

        if let Some(duration) = expire {
            for (key, _) in items {
                #[allow(clippy::cast_possible_truncation)]
                self.pipe.expire(key, duration.as_secs() as usize).ignore();
            }
        }
    }

    pub fn sadd(&mut self, key: impl RedisKey, member: impl ToRedisArgs) {
        self.pipe.sadd(key, member).ignore();
    }

    pub fn scard(&mut self, key: impl RedisKey) {
        self.pipe.scard(key);
    }

    pub fn set(&mut self, key: impl RedisKey, bytes: &[u8], expire: Option<Duration>) {
        if let Some(duration) = expire {
            #[allow(clippy::cast_possible_truncation)]
            self.pipe.set_ex(key, bytes, duration.as_secs() as usize);
        } else {
            self.pipe.set(key, bytes);
        }

        self.pipe.ignore();
    }

    pub fn smembers(&mut self, key: impl RedisKey) {
        self.pipe.smembers(key);
    }

    pub fn srem(&mut self, key: impl RedisKey, member: impl ToRedisArgs) {
        self.pipe.srem(key, member).ignore();
    }

    pub fn zadd(&mut self, key: impl RedisKey, member: impl ToRedisArgs, score: impl ToRedisArgs) {
        self.pipe.zadd(key, member, score).ignore();
    }

    pub fn zrem(&mut self, key: impl RedisKey, members: impl ToRedisArgs) {
        self.pipe.zrem(key, members).ignore();
    }
}

impl<C: CacheConfig> Pipe<'_, C> {
    #[instrument(level = "trace", skip_all)]
    pub async fn get<T>(&mut self, key: impl RedisKey) -> CacheResult<Option<CachedArchive<T>>>
    where
        T: CheckedArchived,
    {
        let conn = self.conn.get().await?;

        let BytesWrap::<AlignedVec<16>>(bytes) = Cmd::get(key).query_async(conn).await?;

        if bytes.is_empty() {
            return Ok(None);
        }

        #[cfg(feature = "bytecheck")]
        let res = CachedArchive::new(bytes).map_err(crate::error::CacheError::Validation);

        #[cfg(not(feature = "bytecheck"))]
        let res = Ok(CachedArchive::new_unchecked(bytes));

        res.map(Some)
    }
}
