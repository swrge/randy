use std::{
    borrow::Cow,
    fmt::{Debug, Formatter, Result as FmtResult},
    ops::Deref,
    time::Duration,
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
    id::{Id, marker::StickerMarker},
};
use redlight::{
    CachedArchive, RedisCache,
    config::{CacheConfig, Cacheable, ICachedGuild, ICachedSticker, Ignore},
    error::CacheError,
    rkyv_util::{
        flags::BitflagsRkyv,
        guild::{AfkTimeoutRkyv, GuildFeatureRkyv},
        id::IdRkyv,
        util::RkyvAsU8,
    },
};
use rkyv::{
    Archive, Archived, Deserialize, Serialize,
    rancor::Source,
    ser::writer::Buffer,
    util::{Align, AlignedVec},
    with::Map,
};

use super::{channel::text_channel, sticker::stickers};

#[derive(Archive, Serialize, Deserialize)]
pub struct CachedGuild {
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

    fn on_guild_update<E: Source>()
    -> Option<fn(&mut CachedArchive<Archived<Self>>, &GuildUpdate) -> Result<(), E>> {
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
            && self.default_message_notifications == u8::from(other.default_message_notifications)
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
