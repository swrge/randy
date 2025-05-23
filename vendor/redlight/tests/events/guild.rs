use std::{
    borrow::Cow,
    fmt::{Debug, Formatter, Result as FmtResult},
    ops::Deref,
    time::Duration,
};

use redlight::{
    config::{CacheConfig, Cacheable, ICachedGuild, ICachedSticker, Ignore},
    error::CacheError,
    rkyv_util::{
        flags::BitflagsRkyv,
        guild::{AfkTimeoutRkyv, GuildFeatureRkyv},
        id::IdRkyv,
        util::RkyvAsU8,
    },
    CachedArchive, RedisCache,
};
use rkyv::{
    rancor::Source,
    ser::writer::Buffer,
    util::{Align, AlignedVec},
    with::Map,
    Archive, Archived, Deserialize, Serialize,
};
use randy_model::{
    channel::message::Sticker,
    gateway::{
        event::Event,
        payload::incoming::{GuildCreate, GuildUpdate},
    },
    guild::{
        AfkTimeout, DefaultMessageNotificationLevel, ExplicitContentFilter, Guild, GuildFeature,
        MfaLevel, NSFWLevel, PartialGuild, Permissions, PremiumTier, SystemChannelFlags,
        VerificationLevel,
    },
    id::{marker::StickerMarker, Id},
};

use super::{channel::text_channel, sticker::stickers};
use crate::pool;

#[tokio::test]
async fn test_guild() -> Result<(), CacheError> {
    struct Config;

    impl CacheConfig for Config {
        #[cfg(feature = "metrics")]
        const METRICS_INTERVAL_DURATION: Duration = Duration::from_secs(60);

        type Channel<'a> = Ignore;
        type CurrentUser<'a> = Ignore;
        type Emoji<'a> = Ignore;
        type Guild<'a> = CachedGuild;
        type Integration<'a> = Ignore;
        type Member<'a> = Ignore;
        type Message<'a> = Ignore;
        type Presence<'a> = Ignore;
        type Role<'a> = Ignore;
        type ScheduledEvent<'a> = Ignore;
        type StageInstance<'a> = Ignore;
        type Sticker<'a> = CachedSticker;
        type User<'a> = Ignore;
        type VoiceState<'a> = Ignore;
    }

    #[derive(Archive, Serialize, Deserialize)]
    struct CachedGuild {
        #[rkyv(with = AfkTimeoutRkyv)]
        afk_timeout: AfkTimeout,
        #[rkyv(with = RkyvAsU8)]
        default_message_notifications: DefaultMessageNotificationLevel,
        #[rkyv(with = RkyvAsU8)]
        explicit_content_filter: ExplicitContentFilter,
        #[rkyv(with = Map<GuildFeatureRkyv>)]
        features: Vec<GuildFeature>,
        #[rkyv(with = RkyvAsU8)]
        mfa_level: MfaLevel,
        #[rkyv(with = RkyvAsU8)]
        nsfw_level: NSFWLevel,
        #[rkyv(with = Map<BitflagsRkyv>)]
        permissions: Option<Permissions>,
        #[rkyv(with = RkyvAsU8)]
        premium_tier: PremiumTier,
        #[rkyv(with = BitflagsRkyv)]
        system_channel_flags: SystemChannelFlags,
        #[rkyv(with = RkyvAsU8)]
        verification_level: VerificationLevel,
    }

    impl<'a> ICachedGuild<'a> for CachedGuild {
        fn from_guild(guild: &'a Guild) -> Self {
            Self {
                afk_timeout: guild.afk_timeout,
                default_message_notifications: guild.default_message_notifications,
                explicit_content_filter: guild.explicit_content_filter,
                features: guild.features.to_owned(),
                mfa_level: guild.mfa_level,
                nsfw_level: guild.nsfw_level,
                permissions: guild.permissions,
                premium_tier: guild.premium_tier,
                system_channel_flags: guild.system_channel_flags,
                verification_level: guild.verification_level,
            }
        }

        fn on_guild_update<E: Source>(
        ) -> Option<fn(&mut CachedArchive<Archived<Self>>, &GuildUpdate) -> Result<(), E>> {
            Some(|archived, update| {
                archived
                    .update_by_deserializing(
                        |deserialized| {
                            deserialized.afk_timeout = update.afk_timeout;
                            deserialized.default_message_notifications =
                                update.default_message_notifications;
                            deserialized.explicit_content_filter = update.explicit_content_filter;
                            deserialized.features = update.features.to_owned();
                            deserialized.mfa_level = update.mfa_level;
                            deserialized.nsfw_level = update.nsfw_level;
                            deserialized.permissions = update.permissions;
                            deserialized.premium_tier = update.premium_tier;
                            deserialized.system_channel_flags = update.system_channel_flags;
                            deserialized.verification_level = update.verification_level;
                        },
                        &mut (),
                    )
                    .map_err(Source::new)
            })
        }
    }

    impl Cacheable for CachedGuild {
        type Bytes = AlignedVec;

        fn expire() -> Option<Duration> {
            None
        }

        fn serialize_one<E: Source>(&self) -> Result<Self::Bytes, E> {
            let mut bytes = AlignedVec::new();
            self.serialize_into(&mut bytes)?;

            Ok(bytes)
        }

        fn serialize_into<E: Source, const N: usize>(
            &self,
            bytes: &mut AlignedVec<N>,
        ) -> Result<(), E> {
            rkyv::api::high::to_bytes_in(self, bytes)?;

            Ok(())
        }
    }

    impl PartialEq<Guild> for ArchivedCachedGuild {
        fn eq(&self, other: &Guild) -> bool {
            u16::from(self.afk_timeout) == other.afk_timeout.get()
                && self.default_message_notifications
                    == u8::from(other.default_message_notifications)
                && self.explicit_content_filter == u8::from(other.explicit_content_filter)
                && self
                    .features
                    .iter()
                    .zip(other.features.iter())
                    .all(|(this, that)| this == Cow::from(that.clone()).as_ref())
                && self.mfa_level == u8::from(other.mfa_level)
                && self.nsfw_level == u8::from(other.nsfw_level)
                && self.permissions == other.permissions
                && self.premium_tier == u8::from(other.premium_tier)
                && self.system_channel_flags == other.system_channel_flags
                && self.verification_level == u8::from(other.verification_level)
        }
    }

    impl Debug for ArchivedCachedGuild {
        fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
            f.debug_struct("ArchivedCachedGuild")
                .field("afk_timeout", &self.afk_timeout)
                .field(
                    "default_message_notifications",
                    &self.default_message_notifications,
                )
                .field("explicit_content_filter", &self.explicit_content_filter)
                .field("features", &self.features)
                .field("mfa_level", &self.mfa_level)
                .field("nsfw_level", &self.nsfw_level)
                .field("permissions", &self.permissions)
                .field("premium_tier", &self.premium_tier)
                .field("system_channel_flags", &self.system_channel_flags)
                .field("verification_level", &self.verification_level)
                .finish()
        }
    }

    #[derive(Archive, Serialize)]
    struct CachedSticker {
        #[rkyv(with = IdRkyv)]
        id: Id<StickerMarker>,
    }

    impl<'a> ICachedSticker<'a> for CachedSticker {
        fn from_sticker(sticker: &'a Sticker) -> Self {
            Self { id: sticker.id }
        }
    }

    impl Cacheable for CachedSticker {
        type Bytes = [u8; 8];

        fn expire() -> Option<Duration> {
            None
        }

        fn serialize_one<E: Source>(&self) -> Result<Self::Bytes, E> {
            let mut bytes = Align([0_u8; 8]);
            rkyv::api::high::to_bytes_in(self, Buffer::from(&mut *bytes))?;

            Ok(bytes.0)
        }
    }

    let mut expected = guild();

    let cache = RedisCache::<Config>::new_with_pool(pool()).await?;

    let guild_create = Event::GuildCreate(Box::new(GuildCreate::Available(expected.clone())));
    cache.update(&guild_create).await?;

    let guild = cache.guild(expected.id).await?.expect("missing guild");
    assert_eq!(guild.deref(), &expected);

    let sticker_ids = cache.guild_sticker_ids(expected.id).await?;
    assert_eq!(expected.stickers.len(), sticker_ids.len());
    assert!(expected
        .stickers
        .iter()
        .all(|sticker| sticker_ids.contains(&sticker.id)));

    let partial = partial_guild();
    expected.afk_timeout = partial.afk_timeout;
    expected.default_message_notifications = partial.default_message_notifications;
    expected.explicit_content_filter = partial.explicit_content_filter;
    expected.features = partial.features.clone();
    expected.mfa_level = partial.mfa_level;
    expected.nsfw_level = partial.nsfw_level;
    expected.permissions = partial.permissions;
    expected.premium_tier = partial.premium_tier;
    expected.system_channel_flags = partial.system_channel_flags;
    expected.verification_level = partial.verification_level;

    let guild_update = Event::GuildUpdate(Box::new(GuildUpdate(partial)));
    cache.update(&guild_update).await?;

    let guild = cache.guild(expected.id).await?.expect("missing guild");
    assert_eq!(guild.deref(), &expected);

    Ok(())
}

pub fn guild() -> Guild {
    Guild {
        afk_channel_id: None,
        afk_timeout: AfkTimeout::ONE_HOUR,
        application_id: None,
        approximate_member_count: None,
        approximate_presence_count: None,
        banner: None,
        channels: vec![text_channel()],
        default_message_notifications: DefaultMessageNotificationLevel::Mentions,
        description: None,
        discovery_splash: None,
        emojis: Vec::new(),
        explicit_content_filter: ExplicitContentFilter::AllMembers,
        features: vec![GuildFeature::Community, GuildFeature::Featurable],
        guild_scheduled_events: Vec::new(),
        icon: None,
        id: Id::new(776),
        joined_at: None,
        large: false,
        max_members: None,
        max_presences: None,
        max_stage_video_channel_users: None,
        max_video_channel_users: None,
        member_count: None,
        members: Vec::new(),
        mfa_level: MfaLevel::Elevated,
        name: "guild name".to_owned(),
        nsfw_level: NSFWLevel::Explicit,
        owner_id: Id::new(189),
        owner: None,
        permissions: Some(Permissions::ADMINISTRATOR | Permissions::SPEAK),
        preferred_locale: "en-US".to_owned(),
        premium_progress_bar_enabled: false,
        premium_subscription_count: None,
        premium_tier: PremiumTier::Tier1,
        presences: Vec::new(),
        public_updates_channel_id: None,
        roles: Vec::new(),
        rules_channel_id: None,
        safety_alerts_channel_id: Some(Id::new(232)),
        splash: None,
        stage_instances: Vec::new(),
        stickers: stickers(),
        system_channel_flags: SystemChannelFlags::SUPPRESS_GUILD_REMINDER_NOTIFICATIONS,
        system_channel_id: None,
        threads: Vec::new(),
        unavailable: Some(false),
        vanity_url_code: None,
        verification_level: VerificationLevel::Medium,
        voice_states: Vec::new(),
        widget_channel_id: None,
        widget_enabled: None,
    }
}

pub fn partial_guild() -> PartialGuild {
    PartialGuild {
        afk_channel_id: None,
        afk_timeout: AfkTimeout::THIRTY_MINUTES,
        application_id: None,
        banner: None,
        default_message_notifications: DefaultMessageNotificationLevel::All,
        description: None,
        discovery_splash: None,
        emojis: Vec::new(),
        explicit_content_filter: ExplicitContentFilter::None,
        features: vec![
            GuildFeature::Community,
            GuildFeature::Featurable,
            GuildFeature::InviteSplash,
        ],
        icon: None,
        id: Id::new(776),
        max_members: None,
        max_presences: None,
        member_count: None,
        mfa_level: MfaLevel::None,
        name: "guild name".to_owned(),
        nsfw_level: NSFWLevel::AgeRestricted,
        owner_id: Id::new(189),
        owner: None,
        permissions: Some(Permissions::ADD_REACTIONS),
        preferred_locale: "en-US".to_owned(),
        premium_progress_bar_enabled: false,
        premium_subscription_count: None,
        premium_tier: PremiumTier::Tier2,
        public_updates_channel_id: None,
        roles: Vec::new(),
        rules_channel_id: None,
        splash: None,
        system_channel_flags: SystemChannelFlags::SUPPRESS_PREMIUM_SUBSCRIPTIONS
            | SystemChannelFlags::SUPPRESS_ROLE_SUBSCRIPTION_PURCHASE_NOTIFICATION_REPLIES,
        system_channel_id: None,
        verification_level: VerificationLevel::High,
        vanity_url_code: None,
        widget_channel_id: None,
        widget_enabled: None,
    }
}
