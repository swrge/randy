use std::{
    fmt::{Debug, Formatter, Result as FmtResult},
    ops::Deref,
    time::Duration,
};

use redlight::{
    config::{CacheConfig, Cacheable, ICachedMember, Ignore},
    error::CacheError,
    rkyv_util::flags::BitflagsRkyv,
    CachedArchive, RedisCache,
};
use rkyv::{
    rancor::Source, ser::writer::Buffer, util::Align, Archive, Archived, Deserialize, Serialize,
};
use randy_model::{
    gateway::{
        event::Event,
        payload::incoming::{MemberAdd, MemberUpdate, MessageCreate},
    },
    guild::{Member, MemberFlags, PartialMember},
    id::{marker::GuildMarker, Id},
    util::Timestamp,
};

use super::user::user;
use crate::{events::message::message, pool};

#[tokio::test]
async fn test_member() -> Result<(), CacheError> {
    struct Config;

    impl CacheConfig for Config {
        #[cfg(feature = "metrics")]
        const METRICS_INTERVAL_DURATION: Duration = Duration::from_secs(60);

        type Channel<'a> = Ignore;
        type CurrentUser<'a> = Ignore;
        type Emoji<'a> = Ignore;
        type Guild<'a> = Ignore;
        type Integration<'a> = Ignore;
        type Member<'a> = CachedMember;
        type Message<'a> = Ignore;
        type Presence<'a> = Ignore;
        type Role<'a> = Ignore;
        type ScheduledEvent<'a> = Ignore;
        type StageInstance<'a> = Ignore;
        type Sticker<'a> = Ignore;
        type User<'a> = Ignore;
        type VoiceState<'a> = Ignore;
    }

    #[derive(Archive, Serialize, Deserialize)]
    struct CachedMember {
        #[rkyv(with = BitflagsRkyv)]
        flags: MemberFlags,
        pending: bool,
    }

    impl<'a> ICachedMember<'a> for CachedMember {
        fn from_member(_: Id<GuildMarker>, member: &'a Member) -> Self {
            Self {
                flags: member.flags,
                pending: member.pending,
            }
        }

        fn on_member_update<E: Source>(
        ) -> Option<fn(&mut CachedArchive<Archived<Self>>, &MemberUpdate) -> Result<(), E>>
        {
            Some(|archived, update| {
                archived
                    .update_by_deserializing(
                        |deserialized| deserialized.pending = update.pending,
                        &mut (),
                    )
                    .map_err(Source::new)
            })
        }

        fn update_via_partial<E: Source>(
        ) -> Option<fn(&mut CachedArchive<Archived<Self>>, &PartialMember) -> Result<(), E>>
        {
            Some(|archived, member| {
                archived.update_archive(|sealed| {
                    rkyv::munge::munge!(let ArchivedCachedMember { mut flags, .. } = sealed);
                    *flags = member.flags.into();
                });

                Ok(())
            })
        }
    }

    impl Cacheable for CachedMember {
        type Bytes = [u8; 16];

        fn expire() -> Option<Duration> {
            None
        }

        fn serialize_one<E: Source>(&self) -> Result<Self::Bytes, E> {
            let mut bytes = Align([0_u8; 16]);
            rkyv::api::high::to_bytes_in(self, Buffer::from(&mut *bytes))?;

            Ok(bytes.0)
        }
    }

    impl PartialEq<Member> for ArchivedCachedMember {
        fn eq(&self, other: &Member) -> bool {
            self.flags == other.flags && self.pending == other.pending
        }
    }

    impl Debug for ArchivedCachedMember {
        fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
            f.debug_struct("ArchivedCachedMember")
                .field("flags", &self.flags)
                .field("pending", &self.pending)
                .finish()
        }
    }

    let mut expected_member = member();
    let expected_update = member_update();
    let expected_partial = partial_member();

    let guild_id = expected_update.guild_id;

    assert_ne!(expected_member.pending, expected_update.pending);
    assert_ne!(expected_member.flags, expected_partial.flags);

    let cache = RedisCache::<Config>::new_with_pool(pool()).await?;

    let member_create = Event::MemberAdd(Box::new(MemberAdd {
        guild_id,
        member: expected_member.clone(),
    }));

    cache.update(&member_create).await?;

    let member = cache
        .member(guild_id, expected_member.user.id)
        .await?
        .expect("missing member");

    assert_eq!(member.deref(), &expected_member);

    expected_member.pending = expected_update.pending;
    let member_update = Event::MemberUpdate(Box::new(expected_update));

    cache.update(&member_update).await?;

    let member = cache
        .member(guild_id, expected_member.user.id)
        .await?
        .expect("missing member");

    assert_eq!(member.deref(), &expected_member);

    expected_member.flags = expected_partial.flags;

    let message_create = Event::MessageCreate(Box::new(MessageCreate(message())));

    cache.update(&message_create).await?;

    let member = cache
        .member(guild_id, expected_member.user.id)
        .await?
        .expect("missing member");

    assert_eq!(member.deref(), &expected_member);

    let mut iter = cache.iter().guild_members(guild_id).await?;

    let member_res = iter.next().expect("missing member");

    #[cfg(feature = "bytecheck")]
    let member = member_res?;

    #[cfg(not(feature = "bytecheck"))]
    let member = member_res;

    assert_eq!(member.deref(), &expected_member);

    assert!(iter.next().is_none());

    Ok(())
}

pub fn member() -> Member {
    Member {
        avatar: None,
        communication_disabled_until: None,
        deaf: false,
        flags: MemberFlags::COMPLETED_ONBOARDING,
        joined_at: Some(Timestamp::parse("2021-01-01T01:01:01+00:00").unwrap()),
        mute: false,
        nick: None,
        pending: true,
        premium_since: None,
        roles: vec![Id::new(123), Id::new(456)],
        user: user(),
    }
}

pub fn member_update() -> MemberUpdate {
    MemberUpdate {
        avatar: None,
        communication_disabled_until: None,
        guild_id: Id::new(111),
        deaf: None,
        flags: None,
        joined_at: Some(Timestamp::parse("2021-01-01T01:01:01+00:00").unwrap()),
        mute: None,
        nick: None,
        pending: false,
        premium_since: None,
        roles: vec![Id::new(123), Id::new(456)],
        user: user(),
    }
}

pub fn partial_member() -> PartialMember {
    PartialMember {
        avatar: None,
        communication_disabled_until: None,
        deaf: false,
        flags: MemberFlags::empty(),
        joined_at: Some(Timestamp::parse("2021-01-01T01:01:01+00:00").unwrap()),
        mute: false,
        nick: None,
        permissions: None,
        premium_since: None,
        roles: vec![Id::new(123), Id::new(456)],
        user: Some(user()),
    }
}
