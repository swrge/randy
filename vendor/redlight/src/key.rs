use std::borrow::Cow;

use itoa::Buffer;
use randy_model::id::{
    marker::{
        ChannelMarker, EmojiMarker, GuildMarker, IntegrationMarker, MessageMarker, RoleMarker,
        ScheduledEventMarker, StageMarker, StickerMarker, UserMarker,
    },
    Id,
};

use crate::redis::{RedisWrite, ToRedisArgs};

/// Keys for storing and loading data from redis.
///
/// Implements `redis::ToRedisArgs` so it can be passed as argument
/// to `redis` commands.
pub trait RedisKey: ToRedisArgs {
    /// The prefix for the Redis key.
    const PREFIX: &'static [u8];
}

pub fn name_id<T>(name: &[u8], id: Id<T>) -> Cow<'static, [u8]> {
    fn inner(name: &[u8], id: u64) -> Cow<'static, [u8]> {
        let mut buf = Buffer::new();
        let id = buf.format(id).as_bytes();

        let mut vec = Vec::with_capacity(name.len() + 1 + id.len());
        vec.extend_from_slice(name);
        vec.push(b':');
        vec.extend_from_slice(id);

        Cow::Owned(vec)
    }

    inner(name, id.get())
}

pub fn name_guild_id<T>(name: &[u8], guild: Id<GuildMarker>, id: Id<T>) -> Cow<'static, [u8]> {
    fn inner(name: &[u8], guild: Id<GuildMarker>, id: u64) -> Cow<'static, [u8]> {
        let mut buf = Buffer::new();
        let guild = buf.format(guild.get()).as_bytes();

        let mut vec = Vec::with_capacity(name.len() + 1 + (guild.len() + 1) * 2);
        vec.extend_from_slice(name);
        vec.push(b':');
        vec.extend_from_slice(guild);
        vec.push(b':');
        let id = buf.format(id).as_bytes();
        vec.extend_from_slice(id);

        Cow::Owned(vec)
    }

    inner(name, guild, id.get())
}
