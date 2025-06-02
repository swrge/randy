use crate::cache::impls::unavailable_guilds::UnavailableGuildsKey;
use crate::cache::meta::atoi;
use crate::cache::meta::IMetaKey;
use crate::config::Cacheable;
use crate::config::ICachedGuild;
use crate::error::CacheError;
use crate::error::ExpireError;
use crate::error::UpdateError;
use crate::error::UpdateErrorKind;
use crate::redis::DedicatedConnection;
use randy_model::gateway::payload::incoming::GuildUpdate;
use randy_model::{
    guild::Guild,
    id::{marker::GuildMarker, Id},
};
use rkyv::Archived;
use std::vec::IntoIter;
use tracing::{instrument, trace};

use crate::{
    cache::pipe::Pipe,
    config::CacheConfig,
    error::{SerializeError, SerializeErrorKind},
    key::{name_id, RedisKey},
    redis::{Pipeline, RedisWrite, ToRedisArgs},
    CacheResult, RedisCache,
};

use super::channel::ChannelKey;
use super::channel::ChannelMetaKey;
use super::channel::ChannelsKey;
use super::emoji::EmojiKey;
use super::emoji::EmojiMetaKey;
use super::member::MemberKey;
use super::presence::PresenceKey;
use super::role::RoleMetaKey;
use super::stage_instance::StageInstanceMetaKey;
use super::user::UserKey;
use super::{
    emoji::EmojisKey,
    integration::IntegrationKey,
    role::{RoleKey, RolesKey},
    scheduled_event::{ScheduledEventKey, ScheduledEventMetaKey, ScheduledEventsKey},
    stage_instance::{StageInstanceKey, StageInstancesKey},
    sticker::{StickerKey, StickerMetaKey, StickersKey},
    user::{UserGuildsKey, UsersKey},
    voice_state::VoiceStateKey,
};

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct GuildKey {
    pub id: Id<GuildMarker>,
}

impl RedisKey for GuildKey {
    const PREFIX: &'static [u8] = b"GUILD";
}

impl ToRedisArgs for GuildKey {
    fn write_redis_args<W>(&self, out: &mut W)
    where
        W: ?Sized + RedisWrite,
    {
        out.write_arg(name_id(Self::PREFIX, self.id).as_ref());
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct GuildChannelsKey {
    pub id: Id<GuildMarker>,
}

impl RedisKey for GuildChannelsKey {
    const PREFIX: &'static [u8] = b"GUILD_CHANNELS";
}

impl ToRedisArgs for GuildChannelsKey {
    fn write_redis_args<W>(&self, out: &mut W)
    where
        W: ?Sized + RedisWrite,
    {
        out.write_arg(name_id(Self::PREFIX, self.id).as_ref());
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct GuildEmojisKey {
    pub id: Id<GuildMarker>,
}

impl RedisKey for GuildEmojisKey {
    const PREFIX: &'static [u8] = b"GUILD_EMOJIS";
}

impl ToRedisArgs for GuildEmojisKey {
    fn write_redis_args<W>(&self, out: &mut W)
    where
        W: ?Sized + RedisWrite,
    {
        out.write_arg(name_id(Self::PREFIX, self.id).as_ref());
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct GuildIntegrationsKey {
    pub id: Id<GuildMarker>,
}

impl RedisKey for GuildIntegrationsKey {
    const PREFIX: &'static [u8] = b"GUILD_INTEGRATIONS";
}

impl ToRedisArgs for GuildIntegrationsKey {
    fn write_redis_args<W>(&self, out: &mut W)
    where
        W: ?Sized + RedisWrite,
    {
        out.write_arg(name_id(Self::PREFIX, self.id).as_ref());
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct GuildMembersKey {
    pub id: Id<GuildMarker>,
}

impl RedisKey for GuildMembersKey {
    const PREFIX: &'static [u8] = b"GUILD_MEMBERS";
}

impl ToRedisArgs for GuildMembersKey {
    fn write_redis_args<W>(&self, out: &mut W)
    where
        W: ?Sized + RedisWrite,
    {
        out.write_arg(name_id(Self::PREFIX, self.id).as_ref());
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct GuildPresencesKey {
    pub id: Id<GuildMarker>,
}

impl RedisKey for GuildPresencesKey {
    const PREFIX: &'static [u8] = b"GUILD_PRESENCES";
}

impl ToRedisArgs for GuildPresencesKey {
    fn write_redis_args<W>(&self, out: &mut W)
    where
        W: ?Sized + RedisWrite,
    {
        out.write_arg(name_id(Self::PREFIX, self.id).as_ref());
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct GuildRolesKey {
    pub id: Id<GuildMarker>,
}

impl RedisKey for GuildRolesKey {
    const PREFIX: &'static [u8] = b"GUILD_ROLES";
}

impl ToRedisArgs for GuildRolesKey {
    fn write_redis_args<W>(&self, out: &mut W)
    where
        W: ?Sized + RedisWrite,
    {
        out.write_arg(name_id(Self::PREFIX, self.id).as_ref());
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct GuildScheduledEventsKey {
    pub id: Id<GuildMarker>,
}

impl RedisKey for GuildScheduledEventsKey {
    const PREFIX: &'static [u8] = b"GUILD_SCHEDULED_EVENTS";
}

impl ToRedisArgs for GuildScheduledEventsKey {
    fn write_redis_args<W>(&self, out: &mut W)
    where
        W: ?Sized + RedisWrite,
    {
        out.write_arg(name_id(Self::PREFIX, self.id).as_ref());
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct GuildStageInstancesKey {
    pub id: Id<GuildMarker>,
}

impl RedisKey for GuildStageInstancesKey {
    const PREFIX: &'static [u8] = b"GUILD_STAGE_INSTANCES";
}

impl ToRedisArgs for GuildStageInstancesKey {
    fn write_redis_args<W>(&self, out: &mut W)
    where
        W: ?Sized + RedisWrite,
    {
        out.write_arg(name_id(Self::PREFIX, self.id).as_ref());
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct GuildStickersKey {
    pub id: Id<GuildMarker>,
}

impl RedisKey for GuildStickersKey {
    const PREFIX: &'static [u8] = b"GUILD_STICKERS";
}

impl ToRedisArgs for GuildStickersKey {
    fn write_redis_args<W>(&self, out: &mut W)
    where
        W: ?Sized + RedisWrite,
    {
        out.write_arg(name_id(Self::PREFIX, self.id).as_ref());
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct GuildVoiceStatesKey {
    pub id: Id<GuildMarker>,
}

impl RedisKey for GuildVoiceStatesKey {
    const PREFIX: &'static [u8] = b"GUILD_VOICE_STATES";
}

impl ToRedisArgs for GuildVoiceStatesKey {
    fn write_redis_args<W>(&self, out: &mut W)
    where
        W: ?Sized + RedisWrite,
    {
        out.write_arg(name_id(Self::PREFIX, self.id).as_ref());
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct GuildsKey;

impl RedisKey for GuildsKey {
    const PREFIX: &'static [u8] = b"GUILDS";
}

impl ToRedisArgs for GuildsKey {
    fn write_redis_args<W>(&self, out: &mut W)
    where
        W: ?Sized + RedisWrite,
    {
        out.write_arg(Self::PREFIX);
    }
}

impl From<Id<GuildMarker>> for GuildKey {
    fn from(id: Id<GuildMarker>) -> Self {
        Self { id }
    }
}

impl<C: CacheConfig> RedisCache<C> {
    #[instrument(level = "trace", skip_all)]
    pub(crate) fn store_guild(&self, pipe: &mut Pipe<'_, C>, guild: &Guild) -> CacheResult<()> {
        if C::Guild::WANTED {
            let guild_id = guild.id;
            let key = GuildKey { id: guild_id };
            let guild = C::Guild::from_guild(guild);

            let bytes = guild
                .serialize_one()
                .map_err(|e| SerializeError::new(e, SerializeErrorKind::Guild))?;

            trace!(bytes = bytes.as_ref().len());

            pipe.set(key, bytes.as_ref(), C::Guild::expire());

            let key = GuildsKey;
            pipe.sadd(key, guild_id.get());

            let key = UnavailableGuildsKey;
            pipe.srem(key, guild_id.get());
        }

        self.store_channels(pipe, guild.id, &guild.channels)?;
        self.store_emojis(pipe, guild.id, &guild.emojis)?;
        self.store_members(pipe, guild.id, &guild.members)?;
        self.store_presences(pipe, guild.id, &guild.presences)?;
        self.store_roles(pipe, guild.id, &guild.roles)?;
        self.store_stickers(pipe, guild.id, &guild.stickers)?;
        self.store_channels(pipe, guild.id, &guild.threads)?;
        self.store_scheduled_events(pipe, guild.id, &guild.guild_scheduled_events)?;
        self.store_stage_instances(pipe, guild.id, &guild.stage_instances)?;
        self.store_voice_states(pipe, guild.id, &guild.voice_states)?;

        Ok(())
    }

    #[instrument(level = "trace", skip_all)]
    pub(crate) async fn store_guild_update(
        &self,
        pipe: &mut Pipe<'_, C>,
        update: &GuildUpdate,
    ) -> CacheResult<()> {
        let guild_id = update.id;

        self.store_emojis(pipe, guild_id, &update.emojis)?;
        self.store_roles(pipe, guild_id, &update.roles)?;

        if !C::Guild::WANTED {
            return Ok(());
        }

        let key = GuildsKey;
        pipe.sadd(key, guild_id.get());

        let key = UnavailableGuildsKey;
        pipe.srem(key, guild_id.get());

        let Some(update_fn) = C::Guild::on_guild_update() else {
            return Ok(());
        };

        let key = GuildKey { id: guild_id };

        let Some(mut guild) = pipe.get::<Archived<C::Guild<'static>>>(key).await? else {
            return Ok(());
        };

        update_fn(&mut guild, update).map_err(|e| UpdateError::new(e, UpdateErrorKind::Guild))?;

        let key = GuildKey { id: guild_id };
        let bytes = guild.into_bytes();
        trace!(bytes = bytes.as_ref().len());
        pipe.set(key, &bytes, C::Guild::expire());

        Ok(())
    }

    pub(crate) async fn delete_guild(
        &self,
        pipe: &mut Pipe<'_, C>,
        guild_id: Id<GuildMarker>,
    ) -> CacheResult<()> {
        debug_assert!(pipe.is_empty());

        if C::Member::WANTED || C::User::WANTED {
            let key = GuildMembersKey { id: guild_id };
            pipe.smembers(key);
        }

        if C::Channel::WANTED {
            let key = GuildChannelsKey { id: guild_id };
            pipe.smembers(key);
        }

        if C::Emoji::WANTED {
            let key = GuildEmojisKey { id: guild_id };
            pipe.smembers(key);
        }

        if C::Integration::WANTED {
            let key = GuildIntegrationsKey { id: guild_id };
            pipe.smembers(key);
        }

        if C::Presence::WANTED {
            let key = GuildPresencesKey { id: guild_id };
            pipe.smembers(key);
        }

        if C::Role::WANTED {
            let key = GuildRolesKey { id: guild_id };
            pipe.smembers(key);
        }

        if C::ScheduledEvent::WANTED {
            let key = GuildScheduledEventsKey { id: guild_id };
            pipe.smembers(key);
        }

        if C::StageInstance::WANTED {
            let key = GuildStageInstancesKey { id: guild_id };
            pipe.smembers(key);
        }

        if C::Sticker::WANTED {
            let key = GuildStickersKey { id: guild_id };
            pipe.smembers(key);
        }

        if C::VoiceState::WANTED {
            let key = GuildVoiceStatesKey { id: guild_id };
            pipe.smembers(key);
        }

        if pipe.is_empty() {
            if C::Guild::WANTED {
                let key = GuildKey { id: guild_id };
                pipe.del(key);

                let key = GuildsKey;
                pipe.srem(key, guild_id.get());
            }

            return Ok(());
        }

        let mut iter = pipe.query::<Vec<Vec<u64>>>().await?.into_iter();

        delete_member_user::<C>(pipe, &mut iter, guild_id).await?;
        delete_channel::<C>(pipe, &mut iter, guild_id)?;
        delete_emoji::<C>(pipe, &mut iter, guild_id)?;
        delete_integration::<C>(pipe, &mut iter, guild_id)?;
        delete_presence::<C>(pipe, &mut iter, guild_id)?;
        delete_role::<C>(pipe, &mut iter, guild_id)?;
        delete_event::<C>(pipe, &mut iter, guild_id)?;
        delete_stage::<C>(pipe, &mut iter, guild_id)?;
        delete_sticker::<C>(pipe, &mut iter, guild_id)?;
        delete_voice_state::<C>(pipe, &mut iter, guild_id)?;

        if C::Guild::WANTED {
            let key = GuildKey { id: guild_id };
            pipe.del(key);

            let key = GuildsKey;
            pipe.srem(key, guild_id.get());
        }

        Ok(())
    }

    pub(crate) async fn delete_guilds(
        &self,
        pipe: &mut Pipe<'_, C>,
        guild_ids: &[u64],
    ) -> CacheResult<()> {
        debug_assert!(pipe.is_empty());

        let count = usize::from(C::Channel::WANTED)
            + usize::from(C::Emoji::WANTED)
            + usize::from(C::Integration::WANTED)
            + usize::from(C::Member::WANTED || C::User::WANTED)
            + usize::from(C::Presence::WANTED)
            + usize::from(C::Role::WANTED)
            + usize::from(C::ScheduledEvent::WANTED)
            + usize::from(C::StageInstance::WANTED)
            + usize::from(C::Sticker::WANTED)
            + usize::from(C::VoiceState::WANTED);

        #[allow(clippy::items_after_statements)]
        fn add_smembers_keys<C, F, K>(pipe: &mut Pipe<'_, C>, guild_ids: &[u64], key_fn: F)
        where
            K: RedisKey,
            F: Fn(Id<GuildMarker>) -> K,
        {
            for &guild_id in guild_ids {
                pipe.smembers(key_fn(Id::new(guild_id)));
            }
        }

        if C::Member::WANTED || C::User::WANTED {
            add_smembers_keys(pipe, guild_ids, |id| GuildMembersKey { id });
        }

        if C::Channel::WANTED {
            add_smembers_keys(pipe, guild_ids, |id| GuildChannelsKey { id });
        }

        if C::Emoji::WANTED {
            add_smembers_keys(pipe, guild_ids, |id| GuildEmojisKey { id });
        }

        if C::Integration::WANTED {
            add_smembers_keys(pipe, guild_ids, |id| GuildIntegrationsKey { id });
        }

        if C::Presence::WANTED {
            add_smembers_keys(pipe, guild_ids, |id| GuildPresencesKey { id });
        }

        if C::Role::WANTED {
            add_smembers_keys(pipe, guild_ids, |id| GuildRolesKey { id });
        }

        if C::ScheduledEvent::WANTED {
            add_smembers_keys(pipe, guild_ids, |id| GuildScheduledEventsKey { id });
        }

        if C::StageInstance::WANTED {
            add_smembers_keys(pipe, guild_ids, |id| GuildStageInstancesKey { id });
        }

        if C::Sticker::WANTED {
            add_smembers_keys(pipe, guild_ids, |id| GuildStickersKey { id });
        }

        if C::VoiceState::WANTED {
            add_smembers_keys(pipe, guild_ids, |id| GuildVoiceStatesKey { id });
        }

        if pipe.is_empty() {
            delete_guilds(pipe, guild_ids);
            return Ok(());
        }

        let data = pipe.query::<Vec<Vec<u64>>>().await?;

        if data.len() != count * guild_ids.len() {
            return Err(CacheError::InvalidResponse);
        }

        let mut iter = data.into_iter();

        delete_members_users::<C>(pipe, &mut iter, guild_ids).await?;
        delete_channels::<C>(pipe, &mut iter, guild_ids);
        delete_emojis::<C>(pipe, &mut iter, guild_ids);
        delete_integrations::<C>(pipe, &mut iter, guild_ids);
        delete_presences::<C>(pipe, &mut iter, guild_ids);
        delete_roles::<C>(pipe, &mut iter, guild_ids);
        delete_events::<C>(pipe, &mut iter, guild_ids);
        delete_stages::<C>(pipe, &mut iter, guild_ids);
        delete_stickers::<C>(pipe, &mut iter, guild_ids);
        delete_voice_states::<C>(pipe, &mut iter, guild_ids);

        delete_guilds(pipe, guild_ids);

        Ok(())
    }
}

// Deleting entries of a single guild

async fn delete_member_user<C: CacheConfig>(
    pipe: &mut Pipe<'_, C>,
    iter: &mut IntoIter<Vec<u64>>,
    guild_id: Id<GuildMarker>,
) -> CacheResult<()> {
    if !(C::Member::WANTED || C::User::WANTED) {
        return Ok(());
    }

    let user_ids = iter.next().ok_or(CacheError::InvalidResponse)?;

    if C::User::WANTED {
        for &user_id in user_ids.iter() {
            let user_id = Id::new(user_id);

            let key = UserGuildsKey { id: user_id };
            pipe.srem(key.clone(), guild_id.get());
            pipe.scard(key);
        }

        let scards: Vec<usize> = pipe.query().await?;

        let estranged_user_ids: Vec<u64> = user_ids
            .iter()
            .zip(scards)
            .filter(|(_, common_guild_count)| *common_guild_count == 0)
            .map(|(user_id, _)| *user_id)
            .collect();

        let user_keys = estranged_user_ids
            .iter()
            .map(|user_id| UserKey {
                id: Id::new(*user_id),
            })
            .collect::<Vec<_>>();

        pipe.del(user_keys);

        let key = UsersKey;
        pipe.srem(key, &estranged_user_ids);
    }

    if C::Member::WANTED {
        let key = GuildMembersKey { id: guild_id };
        pipe.del(key);

        let member_keys = user_ids
            .iter()
            .map(|&user_id| MemberKey {
                guild: guild_id,
                user: Id::new(user_id),
            })
            .collect::<Vec<_>>();

        pipe.del(member_keys);
    }

    Ok(())
}

fn delete_channel<C: CacheConfig>(
    pipe: &mut Pipe<'_, C>,
    iter: &mut IntoIter<Vec<u64>>,
    guild_id: Id<GuildMarker>,
) -> CacheResult<()> {
    if !C::Channel::WANTED {
        return Ok(());
    }

    let key = GuildChannelsKey { id: guild_id };
    pipe.del(key);

    let channel_ids = iter.next().ok_or(CacheError::InvalidResponse)?;

    let key = ChannelsKey;
    pipe.srem(key, channel_ids.as_slice());

    if C::Channel::expire().is_some() {
        let channel_keys = channel_ids
            .iter()
            .map(|channel_id| ChannelMetaKey {
                channel: Id::new(*channel_id),
            })
            .collect::<Vec<_>>();

        pipe.del(channel_keys);
    }

    let channel_keys = channel_ids
        .into_iter()
        .map(|channel_id| ChannelKey {
            id: Id::new(channel_id),
        })
        .collect::<Vec<_>>();

    pipe.del(channel_keys);

    Ok(())
}

fn delete_emoji<C: CacheConfig>(
    pipe: &mut Pipe<'_, C>,
    iter: &mut IntoIter<Vec<u64>>,
    guild_id: Id<GuildMarker>,
) -> CacheResult<()> {
    if !C::Emoji::WANTED {
        return Ok(());
    }

    let key = GuildEmojisKey { id: guild_id };
    pipe.del(key);

    let emoji_ids = iter.next().ok_or(CacheError::InvalidResponse)?;

    let key = EmojisKey;
    pipe.srem(key, emoji_ids.as_slice());

    if C::Emoji::expire().is_some() {
        let emoji_keys = emoji_ids
            .iter()
            .map(|emoji_id| EmojiMetaKey {
                emoji: Id::new(*emoji_id),
            })
            .collect::<Vec<_>>();

        pipe.del(emoji_keys);
    }

    let emoji_keys = emoji_ids
        .into_iter()
        .map(|emoji_id| EmojiKey {
            id: Id::new(emoji_id),
        })
        .collect::<Vec<_>>();

    pipe.del(emoji_keys);

    Ok(())
}

fn delete_integration<C: CacheConfig>(
    pipe: &mut Pipe<'_, C>,
    iter: &mut IntoIter<Vec<u64>>,
    guild_id: Id<GuildMarker>,
) -> CacheResult<()> {
    if !C::Integration::WANTED {
        return Ok(());
    }

    let key = GuildIntegrationsKey { id: guild_id };
    pipe.del(key);

    let integration_ids = iter.next().ok_or(CacheError::InvalidResponse)?;

    let integration_keys = integration_ids
        .into_iter()
        .map(|integration_id| IntegrationKey {
            guild: guild_id,
            id: Id::new(integration_id),
        })
        .collect::<Vec<_>>();

    pipe.del(integration_keys);

    Ok(())
}

fn delete_presence<C: CacheConfig>(
    pipe: &mut Pipe<'_, C>,
    iter: &mut IntoIter<Vec<u64>>,
    guild_id: Id<GuildMarker>,
) -> CacheResult<()> {
    if !C::Presence::WANTED {
        return Ok(());
    }

    let key = GuildPresencesKey { id: guild_id };
    pipe.del(key);

    let user_ids = iter.next().ok_or(CacheError::InvalidResponse)?;

    let presence_keys = user_ids
        .into_iter()
        .map(|user_id| PresenceKey {
            guild: guild_id,
            user: Id::new(user_id),
        })
        .collect::<Vec<_>>();

    pipe.del(presence_keys);

    Ok(())
}

fn delete_role<C: CacheConfig>(
    pipe: &mut Pipe<'_, C>,
    iter: &mut IntoIter<Vec<u64>>,
    guild_id: Id<GuildMarker>,
) -> CacheResult<()> {
    if !C::Role::WANTED {
        return Ok(());
    }

    let key = GuildRolesKey { id: guild_id };
    pipe.del(key);

    let role_ids = iter.next().ok_or(CacheError::InvalidResponse)?;

    let key = RolesKey;
    pipe.srem(key, role_ids.as_slice());

    if C::Role::expire().is_some() {
        let role_keys = role_ids
            .iter()
            .map(|role_id| RoleMetaKey {
                id: Id::new(*role_id),
            })
            .collect::<Vec<_>>();

        pipe.del(role_keys);
    }

    let role_keys = role_ids
        .into_iter()
        .map(|role_id| RoleKey {
            id: Id::new(role_id),
        })
        .collect::<Vec<_>>();

    pipe.del(role_keys);

    Ok(())
}

fn delete_event<C: CacheConfig>(
    pipe: &mut Pipe<'_, C>,
    iter: &mut IntoIter<Vec<u64>>,
    guild_id: Id<GuildMarker>,
) -> CacheResult<()> {
    if !C::ScheduledEvent::WANTED {
        return Ok(());
    }

    let key = GuildScheduledEventsKey { id: guild_id };
    pipe.del(key);

    let event_ids = iter.next().ok_or(CacheError::InvalidResponse)?;

    let key = ScheduledEventsKey;
    pipe.srem(key, event_ids.as_slice());

    if C::ScheduledEvent::expire().is_some() {
        let event_keys = event_ids
            .iter()
            .map(|event_id| ScheduledEventMetaKey {
                id: Id::new(*event_id),
            })
            .collect::<Vec<_>>();

        pipe.del(event_keys);
    }

    let event_keys = event_ids
        .into_iter()
        .map(|event_id| ScheduledEventKey {
            id: Id::new(event_id),
        })
        .collect::<Vec<_>>();

    pipe.del(event_keys);

    Ok(())
}

fn delete_stage<C: CacheConfig>(
    pipe: &mut Pipe<'_, C>,
    iter: &mut IntoIter<Vec<u64>>,
    guild_id: Id<GuildMarker>,
) -> CacheResult<()> {
    if !C::StageInstance::WANTED {
        return Ok(());
    }

    let key = GuildStageInstancesKey { id: guild_id };
    pipe.del(key);

    let stage_ids = iter.next().ok_or(CacheError::InvalidResponse)?;

    let key = StageInstancesKey;
    pipe.srem(key, stage_ids.as_slice());

    if C::StageInstance::expire().is_some() {
        let stage_keys = stage_ids
            .iter()
            .map(|stage_id| StageInstanceMetaKey {
                id: Id::new(*stage_id),
            })
            .collect::<Vec<_>>();

        pipe.del(stage_keys);
    }

    let stage_keys = stage_ids
        .into_iter()
        .map(|stage_instance_id| StageInstanceKey {
            id: Id::new(stage_instance_id),
        })
        .collect::<Vec<_>>();

    pipe.del(stage_keys);

    Ok(())
}

fn delete_sticker<C: CacheConfig>(
    pipe: &mut Pipe<'_, C>,
    iter: &mut IntoIter<Vec<u64>>,
    guild_id: Id<GuildMarker>,
) -> CacheResult<()> {
    if !C::Sticker::WANTED {
        return Ok(());
    }

    let key = GuildStickersKey { id: guild_id };
    pipe.del(key);

    let sticker_ids = iter.next().ok_or(CacheError::InvalidResponse)?;

    let key = StickersKey;
    pipe.srem(key, sticker_ids.as_slice());

    if C::Sticker::expire().is_some() {
        let sticker_keys = sticker_ids
            .iter()
            .map(|sticker_id| StickerMetaKey {
                id: Id::new(*sticker_id),
            })
            .collect::<Vec<_>>();

        pipe.del(sticker_keys);
    }

    let sticker_keys = sticker_ids
        .into_iter()
        .map(|sticker_id| StickerKey {
            id: Id::new(sticker_id),
        })
        .collect::<Vec<_>>();

    pipe.del(sticker_keys);

    Ok(())
}

fn delete_voice_state<C: CacheConfig>(
    pipe: &mut Pipe<'_, C>,
    iter: &mut IntoIter<Vec<u64>>,
    guild_id: Id<GuildMarker>,
) -> CacheResult<()> {
    if !C::VoiceState::WANTED {
        return Ok(());
    }

    let key = GuildVoiceStatesKey { id: guild_id };
    pipe.del(key);

    let user_ids = iter.next().ok_or(CacheError::InvalidResponse)?;

    let voice_state_keys = user_ids
        .into_iter()
        .map(|user_id| VoiceStateKey {
            guild: guild_id,
            user: Id::new(user_id),
        })
        .collect::<Vec<_>>();

    pipe.del(voice_state_keys);

    Ok(())
}

// Deleting entries of multiple guilds

async fn delete_members_users<C: CacheConfig>(
    pipe: &mut Pipe<'_, C>,
    iter: &mut IntoIter<Vec<u64>>,
    guild_ids: &[u64],
) -> CacheResult<()> {
    if !(C::Member::WANTED || C::User::WANTED) {
        return Ok(());
    }

    let user_ids_unflattened = &iter.as_slice()[..guild_ids.len()];

    if C::User::WANTED {
        let user_ids: Vec<_> = user_ids_unflattened.iter().flatten().copied().collect();

        for (user_ids, guild_id) in user_ids_unflattened.iter().zip(guild_ids) {
            for &user_id in user_ids {
                let user_id = Id::new(user_id);

                let key = UserGuildsKey { id: user_id };
                pipe.srem(key, guild_id);

                let key = UserGuildsKey { id: user_id };
                pipe.scard(key);
            }
        }

        let scards: Vec<usize> = pipe.query().await?;

        let key = UsersKey;
        pipe.srem(key, &user_ids);

        let user_keys = user_ids
            .iter()
            .zip(scards)
            .filter(|(_, common_guild_count)| *common_guild_count == 0)
            .map(|(user_id, _)| UserKey {
                id: Id::new(*user_id),
            })
            .collect::<Vec<_>>();

        pipe.del(user_keys);
    }

    if C::Member::WANTED {
        let guild_keys = guild_ids
            .iter()
            .copied()
            .map(|guild_id| GuildMembersKey {
                id: Id::new(guild_id),
            })
            .collect::<Vec<_>>();

        pipe.del(guild_keys);

        let member_keys = user_ids_unflattened
            .iter()
            .zip(guild_ids)
            .flat_map(|(user_ids, guild_id)| {
                user_ids.iter().map(|&user_id| MemberKey {
                    guild: Id::new(*guild_id),
                    user: Id::new(user_id),
                })
            })
            .collect::<Vec<_>>();

        pipe.del(member_keys);
    }

    iter.by_ref().take(guild_ids.len()).for_each(|_| ());

    Ok(())
}

fn delete_channels<C: CacheConfig>(
    pipe: &mut Pipe<'_, C>,
    iter: &mut IntoIter<Vec<u64>>,
    guild_ids: &[u64],
) {
    if !C::Channel::WANTED {
        return;
    }

    let channel_ids: Vec<_> = iter.by_ref().take(guild_ids.len()).flatten().collect();

    let key = ChannelsKey;
    pipe.srem(key, channel_ids.as_slice());

    if C::Channel::expire().is_some() {
        let channel_keys = channel_ids
            .iter()
            .map(|channel_id| ChannelMetaKey {
                channel: Id::new(*channel_id),
            })
            .collect::<Vec<_>>();

        pipe.del(channel_keys);
    }

    let channel_keys = channel_ids
        .into_iter()
        .map(|channel_id| ChannelKey {
            id: Id::new(channel_id),
        })
        .collect::<Vec<_>>();

    pipe.del(channel_keys);

    let guild_keys = guild_ids
        .iter()
        .copied()
        .map(|guild_id| GuildChannelsKey {
            id: Id::new(guild_id),
        })
        .collect::<Vec<_>>();

    pipe.del(guild_keys);
}

fn delete_emojis<C: CacheConfig>(
    pipe: &mut Pipe<'_, C>,
    iter: &mut IntoIter<Vec<u64>>,
    guild_ids: &[u64],
) {
    if !C::Emoji::WANTED {
        return;
    }

    let emoji_ids: Vec<_> = iter.by_ref().take(guild_ids.len()).flatten().collect();

    let key = EmojisKey;
    pipe.srem(key, emoji_ids.as_slice());

    if C::Emoji::expire().is_some() {
        let emoji_keys = emoji_ids
            .iter()
            .map(|emoji_id| EmojiMetaKey {
                emoji: Id::new(*emoji_id),
            })
            .collect::<Vec<_>>();

        pipe.del(emoji_keys);
    }

    let emoji_keys = emoji_ids
        .into_iter()
        .map(|emoji_id| EmojiKey {
            id: Id::new(emoji_id),
        })
        .collect::<Vec<_>>();

    pipe.del(emoji_keys);

    let guild_keys = guild_ids
        .iter()
        .copied()
        .map(|guild_id| GuildEmojisKey {
            id: Id::new(guild_id),
        })
        .collect::<Vec<_>>();

    pipe.del(guild_keys);
}

fn delete_integrations<C: CacheConfig>(
    pipe: &mut Pipe<'_, C>,
    iter: &mut IntoIter<Vec<u64>>,
    guild_ids: &[u64],
) {
    if !C::Integration::WANTED {
        return;
    }

    let integration_keys = iter
        .by_ref()
        .take(guild_ids.len())
        .zip(guild_ids)
        .flat_map(|(integration_ids, guild_id)| {
            integration_ids
                .into_iter()
                .map(|integration_id| IntegrationKey {
                    guild: Id::new(*guild_id),
                    id: Id::new(integration_id),
                })
        })
        .collect::<Vec<_>>();

    pipe.del(integration_keys);

    let guild_keys = guild_ids
        .iter()
        .copied()
        .map(|guild_id| GuildIntegrationsKey {
            id: Id::new(guild_id),
        })
        .collect::<Vec<_>>();

    pipe.del(guild_keys);
}

fn delete_presences<C: CacheConfig>(
    pipe: &mut Pipe<'_, C>,
    iter: &mut IntoIter<Vec<u64>>,
    guild_ids: &[u64],
) {
    if !C::Presence::WANTED {
        return;
    }

    let presence_keys = iter
        .by_ref()
        .take(guild_ids.len())
        .zip(guild_ids)
        .flat_map(|(user_ids, guild_id)| {
            user_ids.into_iter().map(|user_id| PresenceKey {
                guild: Id::new(*guild_id),
                user: Id::new(user_id),
            })
        })
        .collect::<Vec<_>>();

    pipe.del(presence_keys);

    let guild_keys = guild_ids
        .iter()
        .copied()
        .map(|guild_id| GuildPresencesKey {
            id: Id::new(guild_id),
        })
        .collect::<Vec<_>>();

    pipe.del(guild_keys);
}

fn delete_roles<C: CacheConfig>(
    pipe: &mut Pipe<'_, C>,
    iter: &mut IntoIter<Vec<u64>>,
    guild_ids: &[u64],
) {
    if !C::Role::WANTED {
        return;
    }

    let role_ids: Vec<_> = iter.by_ref().take(guild_ids.len()).flatten().collect();

    let key = RolesKey;
    pipe.srem(key, role_ids.as_slice());

    if C::Role::expire().is_some() {
        let role_keys = role_ids
            .iter()
            .map(|role_id| RoleMetaKey {
                id: Id::new(*role_id),
            })
            .collect::<Vec<_>>();

        pipe.del(role_keys);
    }

    let role_keys = role_ids
        .into_iter()
        .map(|role_id| RoleKey {
            id: Id::new(role_id),
        })
        .collect::<Vec<_>>();

    pipe.del(role_keys);

    let guild_keys = guild_ids
        .iter()
        .copied()
        .map(|guild_id| GuildRolesKey {
            id: Id::new(guild_id),
        })
        .collect::<Vec<_>>();

    pipe.del(guild_keys);
}

fn delete_events<C: CacheConfig>(
    pipe: &mut Pipe<'_, C>,
    iter: &mut IntoIter<Vec<u64>>,
    guild_ids: &[u64],
) {
    if !C::ScheduledEvent::WANTED {
        return;
    }

    let event_ids: Vec<_> = iter.by_ref().take(guild_ids.len()).flatten().collect();

    let key = ScheduledEventsKey;
    pipe.srem(key, event_ids.as_slice());

    if C::ScheduledEvent::expire().is_some() {
        let event_keys = event_ids
            .iter()
            .map(|event_id| ScheduledEventMetaKey {
                id: Id::new(*event_id),
            })
            .collect::<Vec<_>>();

        pipe.del(event_keys);
    }

    let event_keys = event_ids
        .into_iter()
        .map(|event_id| ScheduledEventKey {
            id: Id::new(event_id),
        })
        .collect::<Vec<_>>();

    pipe.del(event_keys);

    let guild_keys = guild_ids
        .iter()
        .copied()
        .map(|guild_id| GuildScheduledEventsKey {
            id: Id::new(guild_id),
        })
        .collect::<Vec<_>>();

    pipe.del(guild_keys);
}

fn delete_stages<C: CacheConfig>(
    pipe: &mut Pipe<'_, C>,
    iter: &mut IntoIter<Vec<u64>>,
    guild_ids: &[u64],
) {
    if !C::StageInstance::WANTED {
        return;
    }

    let stage_ids: Vec<_> = iter.by_ref().take(guild_ids.len()).flatten().collect();

    let key = StageInstancesKey;
    pipe.srem(key, stage_ids.as_slice());

    if C::StageInstance::expire().is_some() {
        let stage_keys = stage_ids
            .iter()
            .map(|stage_instance_id| StageInstanceMetaKey {
                id: Id::new(*stage_instance_id),
            })
            .collect::<Vec<_>>();

        pipe.del(stage_keys);
    }

    let stage_keys = stage_ids
        .into_iter()
        .map(|stage_instance_id| StageInstanceKey {
            id: Id::new(stage_instance_id),
        })
        .collect::<Vec<_>>();

    pipe.del(stage_keys);

    let guild_keys = guild_ids
        .iter()
        .copied()
        .map(|guild_id| GuildStageInstancesKey {
            id: Id::new(guild_id),
        })
        .collect::<Vec<_>>();

    pipe.del(guild_keys);
}

fn delete_stickers<C: CacheConfig>(
    pipe: &mut Pipe<'_, C>,
    iter: &mut IntoIter<Vec<u64>>,
    guild_ids: &[u64],
) {
    if !C::Sticker::WANTED {
        return;
    }

    let sticker_ids: Vec<_> = iter.by_ref().take(guild_ids.len()).flatten().collect();

    let key = StickersKey;
    pipe.srem(key, sticker_ids.as_slice());

    if C::Sticker::expire().is_some() {
        let sticker_keys = sticker_ids
            .iter()
            .map(|sticker_id| StickerMetaKey {
                id: Id::new(*sticker_id),
            })
            .collect::<Vec<_>>();

        pipe.del(sticker_keys);
    }

    let sticker_keys = sticker_ids
        .into_iter()
        .map(|sticker_id| StickerKey {
            id: Id::new(sticker_id),
        })
        .collect::<Vec<_>>();

    pipe.del(sticker_keys);

    let guild_keys = guild_ids
        .iter()
        .copied()
        .map(|guild_id| GuildStickersKey {
            id: Id::new(guild_id),
        })
        .collect::<Vec<_>>();

    pipe.del(guild_keys);
}

fn delete_voice_states<C: CacheConfig>(
    pipe: &mut Pipe<'_, C>,
    iter: &mut IntoIter<Vec<u64>>,
    guild_ids: &[u64],
) {
    if !C::VoiceState::WANTED {
        return;
    }

    let voice_state_keys = iter
        .by_ref()
        .take(guild_ids.len())
        .zip(guild_ids)
        .flat_map(|(user_ids, guild_id)| {
            user_ids.into_iter().map(|user_id| VoiceStateKey {
                guild: Id::new(*guild_id),
                user: Id::new(user_id),
            })
        })
        .collect::<Vec<_>>();

    pipe.del(voice_state_keys);

    let guild_keys = guild_ids
        .iter()
        .copied()
        .map(|guild_id| GuildVoiceStatesKey {
            id: Id::new(guild_id),
        })
        .collect::<Vec<_>>();

    pipe.del(guild_keys);
}

fn delete_guilds<C: CacheConfig>(pipe: &mut Pipe<'_, C>, guild_ids: &[u64]) {
    if !C::Guild::WANTED {
        return;
    }

    let guild_keys = guild_ids
        .iter()
        .copied()
        .map(|guild_id| GuildKey {
            id: Id::new(guild_id),
        })
        .collect::<Vec<_>>();

    pipe.del(guild_keys);

    let key = GuildsKey;
    pipe.srem(key, guild_ids);
}

#[derive(Debug)]
pub(crate) struct GuildMetaKey {
    guild: Id<GuildMarker>,
}

impl IMetaKey for GuildMetaKey {
    fn parse<'a>(split: &mut impl Iterator<Item = &'a [u8]>) -> Option<Self> {
        split.next().and_then(atoi).map(|guild| Self { guild })
    }

    fn handle_expire(&self, pipe: &mut Pipeline) {
        let key = GuildsKey;
        pipe.srem(key, self.guild.get());
    }
}

impl GuildMetaKey {
    pub(crate) async fn async_handle_expire(
        self,
        pipe: &mut Pipeline,
        conn: &mut DedicatedConnection,
    ) -> Result<(), ExpireError> {
        debug_assert_eq!(pipe.cmd_iter().count(), 0);

        let key = GuildChannelsKey { id: self.guild };
        pipe.smembers(key.clone()).del(key).ignore();

        let key = GuildEmojisKey { id: self.guild };
        pipe.smembers(key.clone()).del(key).ignore();

        let key = GuildIntegrationsKey { id: self.guild };
        pipe.smembers(key.clone()).del(key).ignore();

        let key = GuildMembersKey { id: self.guild };
        pipe.smembers(key.clone()).del(key).ignore();

        let key = GuildPresencesKey { id: self.guild };
        pipe.smembers(key.clone()).del(key).ignore();

        let key = GuildRolesKey { id: self.guild };
        pipe.smembers(key.clone()).del(key).ignore();

        let key = GuildStageInstancesKey { id: self.guild };
        pipe.smembers(key.clone()).del(key).ignore();

        let key = GuildStickersKey { id: self.guild };
        pipe.smembers(key.clone()).del(key).ignore();

        let key = GuildVoiceStatesKey { id: self.guild };
        pipe.smembers(key.clone()).del(key).ignore();

        let mut iter = pipe
            .query_async::<_, Vec<Vec<u64>>>(conn)
            .await
            .map_err(ExpireError::Pipe)?
            .into_iter();

        pipe.clear();

        let channel_ids = iter.next().unwrap_or_default();
        self.handle_channels(pipe, &channel_ids);

        let emoji_ids = iter.next().unwrap_or_default();
        self.handle_emojis(pipe, &emoji_ids);

        let integration_ids = iter.next().unwrap_or_default();
        self.handle_integrations(pipe, &integration_ids);

        let member_ids = iter.next().unwrap_or_default();
        self.handle_members(pipe, conn, member_ids).await?;

        let presence_ids = iter.next().unwrap_or_default();
        self.handle_presences(pipe, &presence_ids);

        let role_ids = iter.next().unwrap_or_default();
        self.handle_roles(pipe, &role_ids);

        let stage_ids = iter.next().unwrap_or_default();
        self.handle_stages(pipe, &stage_ids);

        let sticker_ids = iter.next().unwrap_or_default();
        self.handle_stickers(pipe, &sticker_ids);

        let voice_state_ids = iter.next().unwrap_or_default();
        self.handle_voice_states(pipe, &voice_state_ids);

        Ok(())
    }

    fn handle_channels(&self, pipe: &mut Pipeline, channel_ids: &[u64]) {
        pipe.srem(ChannelsKey, channel_ids).ignore();

        let keys_to_del: Vec<ChannelKey> = channel_ids
            .iter()
            .map(|&id_val| ChannelKey {
                id: Id::new(id_val),
            })
            .collect();
        if !keys_to_del.is_empty() {
            pipe.del(keys_to_del).ignore();
        }

        let meta_keys_to_del: Vec<ChannelMetaKey> = channel_ids
            .iter()
            .map(|&id_val| ChannelMetaKey {
                channel: Id::new(id_val),
            })
            .collect();
        if !meta_keys_to_del.is_empty() {
            pipe.del(meta_keys_to_del).ignore();
        }
    }

    fn handle_emojis(&self, pipe: &mut Pipeline, emoji_ids: &[u64]) {
        pipe.srem(EmojisKey, emoji_ids).ignore();

        let keys_to_del: Vec<EmojiKey> = emoji_ids
            .iter()
            .map(|&id_val| EmojiKey {
                id: Id::new(id_val),
            })
            .collect();
        if !keys_to_del.is_empty() {
            pipe.del(keys_to_del).ignore();
        }

        let meta_keys_to_del: Vec<EmojiMetaKey> = emoji_ids
            .iter()
            .map(|&id_val| EmojiMetaKey {
                emoji: Id::new(id_val),
            })
            .collect();
        if !meta_keys_to_del.is_empty() {
            pipe.del(meta_keys_to_del).ignore();
        }
    }

    fn handle_integrations(&self, pipe: &mut Pipeline, integration_ids: &[u64]) {
        let iter = integration_ids
            .iter()
            .map(|integration| IntegrationKey {
                guild: self.guild,
                id: Id::new(*integration),
            })
            .collect::<Vec<_>>();

        if !iter.is_empty() {
            pipe.del(iter).ignore();
        }
    }

    async fn handle_members(
        &self,
        pipe: &mut Pipeline,
        conn: &mut DedicatedConnection,
        member_ids: Vec<u64>,
    ) -> Result<(), ExpireError> {
        if member_ids.is_empty() {
            return Ok(());
        }

        for user in member_ids.iter() {
            let key = UserGuildsKey { id: Id::new(*user) };
            pipe.srem(key.clone(), self.guild.get()).ignore().scard(key);
        }

        let scards: Vec<usize> = pipe.query_async(conn).await.map_err(ExpireError::Pipe)?;
        pipe.clear();

        let estranged_user_ids: Vec<u64> = member_ids
            .iter()
            .zip(scards)
            .filter(|(_, common_guild_count)| *common_guild_count == 0)
            .map(|(user_id, _)| *user_id)
            .collect();

        if !estranged_user_ids.is_empty() {
            let user_keys_to_del: Vec<UserKey> = estranged_user_ids
                .iter()
                .map(|user_id| UserKey {
                    id: Id::new(*user_id),
                })
                .collect();
            pipe.del(user_keys_to_del).ignore();

            let key = UsersKey;
            pipe.srem(key, &estranged_user_ids).ignore();
        }

        let member_keys_to_del: Vec<MemberKey> = member_ids
            .iter()
            .map(|user| MemberKey {
                guild: self.guild,
                user: Id::new(*user),
            })
            .collect();
        if !member_keys_to_del.is_empty() {
            pipe.del(member_keys_to_del).ignore();
        }

        Ok(())
    }

    fn handle_presences(&self, pipe: &mut Pipeline, user_ids: &[u64]) {
        let iter = user_ids
            .iter()
            .map(|user| PresenceKey {
                guild: self.guild,
                user: Id::new(*user),
            })
            .collect::<Vec<_>>();

        if !iter.is_empty() {
            pipe.del(iter).ignore();
        }
    }

    fn handle_roles(&self, pipe: &mut Pipeline, role_ids: &[u64]) {
        pipe.srem(RolesKey, role_ids).ignore();

        let keys_to_del: Vec<RoleKey> = role_ids
            .iter()
            .map(|&id_val| RoleKey {
                id: Id::new(id_val),
            })
            .collect();
        if !keys_to_del.is_empty() {
            pipe.del(keys_to_del).ignore();
        }

        let meta_keys_to_del: Vec<RoleMetaKey> = role_ids
            .iter()
            .map(|&id_val| RoleMetaKey {
                id: Id::new(id_val),
            })
            .collect();
        if !meta_keys_to_del.is_empty() {
            pipe.del(meta_keys_to_del).ignore();
        }
    }

    fn handle_stages(&self, pipe: &mut Pipeline, stage_ids: &[u64]) {
        pipe.srem(StageInstancesKey, stage_ids).ignore();

        let keys_to_del: Vec<StageInstanceKey> = stage_ids
            .iter()
            .map(|&id_val| StageInstanceKey {
                id: Id::new(id_val),
            })
            .collect();
        if !keys_to_del.is_empty() {
            pipe.del(keys_to_del).ignore();
        }

        let meta_keys_to_del: Vec<StageInstanceMetaKey> = stage_ids
            .iter()
            .map(|&id_val| StageInstanceMetaKey {
                id: Id::new(id_val),
            })
            .collect();
        if !meta_keys_to_del.is_empty() {
            pipe.del(meta_keys_to_del).ignore();
        }
    }

    fn handle_stickers(&self, pipe: &mut Pipeline, sticker_ids: &[u64]) {
        pipe.srem(StickersKey, sticker_ids).ignore();

        let keys_to_del: Vec<StickerKey> = sticker_ids
            .iter()
            .map(|&id_val| StickerKey {
                id: Id::new(id_val),
            })
            .collect();
        if !keys_to_del.is_empty() {
            pipe.del(keys_to_del).ignore();
        }

        let meta_keys_to_del: Vec<StickerMetaKey> = sticker_ids
            .iter()
            .map(|&id_val| StickerMetaKey {
                id: Id::new(id_val),
            })
            .collect();
        if !meta_keys_to_del.is_empty() {
            pipe.del(meta_keys_to_del).ignore();
        }
    }

    fn handle_voice_states(&self, pipe: &mut Pipeline, user_ids: &[u64]) {
        let iter = user_ids
            .iter()
            .map(|user| VoiceStateKey {
                guild: self.guild,
                user: Id::new(*user),
            })
            .collect::<Vec<_>>();

        if !iter.is_empty() {
            pipe.del(iter).ignore();
        }
    }
}
