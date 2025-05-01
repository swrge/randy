//mod channel;
//mod current_user;
//mod guild;
//mod member;
//mod message;
//mod presence;
//mod role;
//mod stickers;
//mod user;

use redlight::config::{CacheConfig, Ignore};

//use self::{
//    channel::CachedChannel, current_user::CachedCurrentUser, guild::CachedGuild,
//    member::CachedMember, message::CachedMessage, presence::CachedPresence, role::CachedRole,
//    stickers::CachedSticker, user::CachedUser,
//};

pub struct RedisConfig;

impl CacheConfig for RedisConfig {
    #[cfg(feature = "metrics")]
    const METRICS_INTERVAL_DURATION: std::time::Duration = std::time::Duration::from_secs(30);

    type Channel<'a> = Ignore;
    type CurrentUser<'a> = Ignore;
    type Emoji<'a> = Ignore;
    type Guild<'a> = Ignore;
    type Integration<'a> = Ignore;
    type Member<'a> = Ignore;
    type Message<'a> = Ignore;
    type Presence<'a> = Ignore;
    type Role<'a> = Ignore;
    type ScheduledEvent<'a> = Ignore;
    type StageInstance<'a> = Ignore;
    type Sticker<'a> = Ignore;
    type User<'a> = Ignore;
    type VoiceState<'a> = Ignore;
}
