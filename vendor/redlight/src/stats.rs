use randy_model::id::{
    marker::{ChannelMarker, GuildMarker, UserMarker},
    Id,
};

use crate::{
    cache::{
        ChannelMessagesKey, ChannelsKey, EmojisKey, GuildChannelsKey, GuildEmojisKey,
        GuildIntegrationsKey, GuildMembersKey, GuildPresencesKey, GuildRolesKey,
        GuildStageInstancesKey, GuildStickersKey, GuildVoiceStatesKey, GuildsKey, MessagesKey,
        RolesKey, StageInstancesKey, StickersKey, UnavailableGuildsKey, UserGuildsKey, UsersKey,
    },
    error::CacheError,
    redis::{Cmd, ConnectionState},
    CacheResult, RedisCache,
};

/// Retrieve the size count of various cached collections.
///
/// Created via [`RedisCache::stats`].
pub struct RedisCacheStats<'c, C> {
    conn: ConnectionState<'c, C>,
}

macro_rules! impl_stats_fn {
    ($doc:literal, $fn:ident, $variant:ident) => {
        #[doc = $doc]
        pub async fn $fn(&mut self) -> CacheResult<usize> {
            let conn = self.conn.get().await?;

            Cmd::scard($variant)
                .query_async(conn)
                .await
                .map_err(CacheError::Redis)
        }
    };
    (Guild: $doc:literal, $fn:ident, $variant:ident) => {
        #[doc = $doc]
        pub async fn $fn(&mut self, guild_id: Id<GuildMarker>) -> CacheResult<usize> {
            let conn = self.conn.get().await?;

            Cmd::scard($variant { id: guild_id })
                .query_async(conn)
                .await
                .map_err(CacheError::Redis)
        }
    };
}

impl<'c, C> RedisCacheStats<'c, C> {
    pub(crate) const fn new(cache: &'c RedisCache<C>) -> RedisCacheStats<'c, C> {
        Self {
            conn: ConnectionState::new(cache),
        }
    }
}

impl<C> RedisCacheStats<'_, C> {
    impl_stats_fn!(
        "Total amount of currently cached channels.",
        channels,
        ChannelsKey
    );

    impl_stats_fn!(
        "Total amount of currently cached emojis.",
        emojis,
        EmojisKey
    );

    impl_stats_fn!(
        "Total amount of currently cached guilds.",
        guilds,
        GuildsKey
    );

    impl_stats_fn!(
        "Total amount of currently cached messages.",
        messages,
        MessagesKey
    );

    impl_stats_fn!("Total amount of currently cached roles.", roles, RolesKey);

    impl_stats_fn!(
        "Total amount of currently cached stage instances.",
        stage_instances,
        StageInstancesKey
    );

    impl_stats_fn!(
        "Total amount of currently cached stickers.",
        stickers,
        StickersKey
    );

    impl_stats_fn!(
        "Total amount of currently unavailable guilds.",
        unavailable_guilds,
        UnavailableGuildsKey
    );

    impl_stats_fn!("Total amount of currently cached users.", users, UsersKey);

    impl_stats_fn!(
        Guild:
       "Amount of currently cached channels for a guild.",
        guild_channels,
        GuildChannelsKey
    );

    impl_stats_fn!(
        Guild:
       "Amount of currently cached emojis for a guild.",
        guild_emojis,
        GuildEmojisKey
    );

    impl_stats_fn!(
        Guild:
       "Amount of currently cached integrations for a guild.",
        guild_integrations,
        GuildIntegrationsKey
    );

    impl_stats_fn!(
        Guild:
       "Amount of currently cached members for a guild.",
        guild_members,
        GuildMembersKey
    );

    impl_stats_fn!(
        Guild:
       "Amount of currently cached presences for a guild.",
        guild_presences,
        GuildPresencesKey
    );

    impl_stats_fn!(
        Guild:
       "Amount of currently cached roles for a guild.",
        guild_roles,
        GuildRolesKey
    );

    impl_stats_fn!(
        Guild:
       "Amount of currently cached stage instances for a guild.",
        guild_stage_instances,
        GuildStageInstancesKey
    );

    impl_stats_fn!(
        Guild:
       "Amount of currently cached stickers for a guild.",
        guild_stickers,
        GuildStickersKey
    );

    impl_stats_fn!(
        Guild:
       "Amount of currently cached voice states for a guild.",
        guild_voice_states,
        GuildVoiceStatesKey
    );

    /// Amount of currently cached messages for a channel.
    pub async fn channel_messages(&mut self, channel_id: Id<ChannelMarker>) -> CacheResult<usize> {
        let conn = self.conn.get().await?;

        let key = ChannelMessagesKey {
            channel: channel_id,
        };

        Cmd::zcard(key)
            .query_async(conn)
            .await
            .map_err(CacheError::Redis)
    }

    /// Amount of known guilds that a user is in.
    pub async fn common_guilds(&mut self, user_id: Id<UserMarker>) -> CacheResult<usize> {
        let conn = self.conn.get().await?;

        Cmd::scard(UserGuildsKey { id: user_id })
            .query_async(conn)
            .await
            .map_err(CacheError::Redis)
    }
}
