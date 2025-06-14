use std::collections::HashSet;

use randy_model::id::{
    marker::{
        ChannelMarker, EmojiMarker, GuildMarker, IntegrationMarker, MessageMarker, RoleMarker,
        StageMarker, StickerMarker, UserMarker,
    },
    Id,
};
use rkyv::{util::AlignedVec, Archived};

use super::Connection;
use crate::{
    config::{CacheConfig, CheckedArchived},
    error::CacheError,
    key::RedisKey,
    redis::{Cmd, FromRedisValue},
    util::{convert_ids_set, convert_ids_vec, BytesWrap},
    CacheResult, CachedArchive, RedisCache,
};

use crate::cache::impls::{
    channel::{ChannelKey, ChannelMessagesKey, ChannelsKey},
    current_user::CurrentUserKey,
    emoji::{EmojiKey, EmojisKey},
    guild::{
        GuildChannelsKey, GuildEmojisKey, GuildIntegrationsKey, GuildKey, GuildMembersKey,
        GuildPresencesKey, GuildRolesKey, GuildScheduledEventsKey, GuildStageInstancesKey,
        GuildStickersKey, GuildVoiceStatesKey, GuildsKey,
    },
    integration::IntegrationKey,
    member::MemberKey,
    message::{MessageKey, MessagesKey},
    presence::PresenceKey,
    role::{RoleKey, RolesKey},
    stage_instance::{StageInstanceKey, StageInstancesKey},
    sticker::{StickerKey, StickersKey},
    unavailable_guilds::UnavailableGuildsKey,
    user::{UserGuildsKey, UserKey, UsersKey},
    voice_state::VoiceStateKey,
};

type GetResult<T> = CacheResult<Option<CachedArchive<Archived<T>>>>;

impl<C: CacheConfig> RedisCache<C> {
    /// Get a channel entry.
    pub async fn channel(&self, channel_id: Id<ChannelMarker>) -> GetResult<C::Channel<'static>> {
        self.get_single(ChannelKey { id: channel_id }).await
    }

    /// Get the current user entry.
    pub async fn current_user(&self) -> GetResult<C::CurrentUser<'static>> {
        self.get_single(CurrentUserKey).await
    }

    /// Get an emoji entry.
    pub async fn emoji(&self, emoji_id: Id<EmojiMarker>) -> GetResult<C::Emoji<'static>> {
        self.get_single(EmojiKey { id: emoji_id }).await
    }

    /// Get a guild entry.
    pub async fn guild(&self, guild_id: Id<GuildMarker>) -> GetResult<C::Guild<'static>> {
        self.get_single(GuildKey { id: guild_id }).await
    }

    /// Get an integration entry.
    pub async fn integration(
        &self,
        guild_id: Id<GuildMarker>,
        integration_id: Id<IntegrationMarker>,
    ) -> GetResult<C::Integration<'static>> {
        let key = IntegrationKey {
            guild: guild_id,
            id: integration_id,
        };

        self.get_single(key).await
    }

    /// Get a member entry.
    pub async fn member(
        &self,
        guild_id: Id<GuildMarker>,
        user_id: Id<UserMarker>,
    ) -> GetResult<C::Member<'static>> {
        let key = MemberKey {
            guild: guild_id,
            user: user_id,
        };

        self.get_single(key).await
    }

    /// Get a message entry.
    pub async fn message(&self, msg_id: Id<MessageMarker>) -> GetResult<C::Message<'static>> {
        self.get_single(MessageKey { id: msg_id }).await
    }

    /// Get a presence entry.
    pub async fn presence(
        &self,
        guild_id: Id<GuildMarker>,
        user_id: Id<UserMarker>,
    ) -> GetResult<C::Presence<'static>> {
        let key = PresenceKey {
            guild: guild_id,
            user: user_id,
        };

        self.get_single(key).await
    }

    /// Get a role entry.
    pub async fn role(&self, role_id: Id<RoleMarker>) -> GetResult<C::Role<'static>> {
        self.get_single(RoleKey { id: role_id }).await
    }

    /// Get a stage instance entry.
    pub async fn stage_instance(
        &self,
        stage_instance_id: Id<StageMarker>,
    ) -> GetResult<C::StageInstance<'static>> {
        self.get_single(StageInstanceKey {
            id: stage_instance_id,
        })
        .await
    }

    /// Get a sticker entry.
    pub async fn sticker(&self, sticker_id: Id<StickerMarker>) -> GetResult<C::Sticker<'static>> {
        self.get_single(StickerKey { id: sticker_id }).await
    }

    /// Get a user entry.
    pub async fn user(&self, user_id: Id<UserMarker>) -> GetResult<C::User<'static>> {
        self.get_single(UserKey { id: user_id }).await
    }

    /// Get a voice state entry.
    pub async fn voice_state(
        &self,
        guild_id: Id<GuildMarker>,
        user_id: Id<UserMarker>,
    ) -> GetResult<C::VoiceState<'static>> {
        let key = VoiceStateKey {
            guild: guild_id,
            user: user_id,
        };

        self.get_single(key).await
    }

    /// Get all cached channel ids.
    pub async fn channel_ids(&self) -> CacheResult<HashSet<Id<ChannelMarker>>> {
        self.get_ids(ChannelsKey).await
    }

    /// Get all cached message ids for a channel.
    ///
    /// The ids are ordered by message timestamp i.e. most recent to oldest.
    pub async fn channel_message_ids(
        &self,
        channel_id: Id<ChannelMarker>,
    ) -> CacheResult<Vec<Id<MessageMarker>>> {
        let mut conn = self.connection().await?;

        let key = ChannelMessagesKey {
            channel: channel_id,
        };

        Cmd::zrange(key, 0, -1)
            .query_async(&mut conn)
            .await
            .map(convert_ids_vec)
            .map_err(CacheError::Redis)
    }

    /// Get all cached guild ids that a user is in.
    pub async fn common_guild_ids(
        &self,
        user_id: Id<UserMarker>,
    ) -> CacheResult<HashSet<Id<GuildMarker>>> {
        self.get_ids(UserGuildsKey { id: user_id }).await
    }

    /// Get all cached emoji ids.
    pub async fn emoji_ids(&self) -> CacheResult<HashSet<Id<EmojiMarker>>> {
        self.get_ids(EmojisKey).await
    }

    /// Get all cached guild ids.
    pub async fn guild_ids(&self) -> CacheResult<HashSet<Id<GuildMarker>>> {
        self.get_ids(GuildsKey).await
    }

    /// Get all cached message ids.
    pub async fn message_ids(&self) -> CacheResult<HashSet<Id<MessageMarker>>> {
        self.get_ids(MessagesKey).await
    }

    /// Get all cached role ids.
    pub async fn role_ids(&self) -> CacheResult<HashSet<Id<RoleMarker>>> {
        self.get_ids(RolesKey).await
    }

    /// Get all cached stage instance ids.
    pub async fn stage_instance_ids(&self) -> CacheResult<HashSet<Id<StageMarker>>> {
        self.get_ids(StageInstancesKey).await
    }

    /// Get all cached sticker ids.
    pub async fn sticker_ids(&self) -> CacheResult<HashSet<Id<StickerMarker>>> {
        self.get_ids(StickersKey).await
    }

    /// Get all currently unavailable guild ids.
    pub async fn unavailable_guild_ids(&self) -> CacheResult<HashSet<Id<GuildMarker>>> {
        self.get_ids(UnavailableGuildsKey).await
    }

    /// Get all cached user ids.
    pub async fn user_ids(&self) -> CacheResult<HashSet<Id<UserMarker>>> {
        self.get_ids(UsersKey).await
    }

    /// Get all cached channel ids for a guild.
    pub async fn guild_channel_ids(
        &self,
        guild_id: Id<GuildMarker>,
    ) -> CacheResult<HashSet<Id<ChannelMarker>>> {
        self.get_ids(GuildChannelsKey { id: guild_id }).await
    }

    /// Get all cached emoji ids for a guild.
    pub async fn guild_emoji_ids(
        &self,
        guild_id: Id<GuildMarker>,
    ) -> CacheResult<HashSet<Id<EmojiMarker>>> {
        self.get_ids(GuildEmojisKey { id: guild_id }).await
    }

    /// Get all cached integration ids for a guild.
    pub async fn guild_integration_ids(
        &self,
        guild_id: Id<GuildMarker>,
    ) -> CacheResult<HashSet<Id<IntegrationMarker>>> {
        self.get_ids(GuildIntegrationsKey { id: guild_id }).await
    }

    /// Get all cached member ids for a guild.
    pub async fn guild_member_ids(
        &self,
        guild_id: Id<GuildMarker>,
    ) -> CacheResult<HashSet<Id<UserMarker>>> {
        self.get_ids(GuildMembersKey { id: guild_id }).await
    }

    /// Get all cached user ids of presences for a guild.
    pub async fn guild_presence_ids(
        &self,
        guild_id: Id<GuildMarker>,
    ) -> CacheResult<HashSet<Id<UserMarker>>> {
        self.get_ids(GuildPresencesKey { id: guild_id }).await
    }

    /// Get all cached role ids for a guild.
    pub async fn guild_role_ids(
        &self,
        guild_id: Id<GuildMarker>,
    ) -> CacheResult<HashSet<Id<RoleMarker>>> {
        self.get_ids(GuildRolesKey { id: guild_id }).await
    }

    /// Get all cached stage instance ids for a guild.
    pub async fn guild_stage_instance_ids(
        &self,
        guild_id: Id<GuildMarker>,
    ) -> CacheResult<HashSet<Id<StageMarker>>> {
        self.get_ids(GuildStageInstancesKey { id: guild_id }).await
    }

    /// Get all cached sticker ids for a guild.
    pub async fn guild_sticker_ids(
        &self,
        guild_id: Id<GuildMarker>,
    ) -> CacheResult<HashSet<Id<StickerMarker>>> {
        self.get_ids(GuildStickersKey { id: guild_id }).await
    }

    /// Get all cached user ids of voice states in a guild.
    pub async fn guild_voice_state_ids(
        &self,
        guild_id: Id<GuildMarker>,
    ) -> CacheResult<HashSet<Id<UserMarker>>> {
        self.get_ids(GuildVoiceStatesKey { id: guild_id }).await
    }
}

impl<C> RedisCache<C> {
    async fn get_single<K, V>(&self, key: K) -> CacheResult<Option<CachedArchive<V>>>
    where
        K: RedisKey,
        V: CheckedArchived,
    {
        let mut conn = self.connection().await?;

        let BytesWrap::<AlignedVec<16>>(bytes) = Cmd::get(key).query_async(&mut conn).await?;

        if bytes.is_empty() {
            return Ok(None);
        }

        #[cfg(feature = "bytecheck")]
        {
            CachedArchive::new(bytes)
                .map_err(CacheError::Validation)
                .map(Some)
        }

        #[cfg(not(feature = "bytecheck"))]
        {
            Ok(Some(CachedArchive::new_unchecked(bytes)))
        }
    }

    async fn get_ids<T>(&self, key: impl RedisKey) -> CacheResult<HashSet<Id<T>>> {
        let mut conn = self.connection().await?;

        Self::get_ids_static(key, &mut conn)
            .await
            .map(convert_ids_set)
    }

    pub(crate) async fn get_ids_static<T>(
        key: impl RedisKey,
        conn: &mut Connection<'_>,
    ) -> CacheResult<T>
    where
        T: FromRedisValue,
    {
        Cmd::smembers(key)
            .query_async(conn)
            .await
            .map_err(CacheError::Redis)
    }
}

#[cfg(test)]
#[cfg(feature = "bytecheck")]
mod tests {
    use std::collections::HashSet;

    use randy_model::id::{marker::GenericMarker, Id};

    use super::convert_ids_set;

    #[test]
    fn test_convert_ids_zero() {
        let mut ids = HashSet::new();
        ids.insert(3);
        ids.insert(0);
        ids.insert(5);
        let converted: HashSet<Id<GenericMarker>> = convert_ids_set(ids);

        assert_eq!(converted.len(), 2);
    }
}
