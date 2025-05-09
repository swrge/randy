use std::{
    borrow::Cow,
    fmt::{Debug, Formatter, Result as FmtResult},
    ops::Deref,
    time::Duration,
};

use redlight::{
    config::{CacheConfig, Cacheable, ICachedIntegration, Ignore},
    error::CacheError,
    rkyv_util::{
        integration::{GuildIntegrationTypeRkyv, IntegrationAccountRkyv},
        util::RkyvAsU8,
    },
    RedisCache,
};
use rkyv::{
    rancor::Source,
    util::AlignedVec,
    with::{InlineAsBox, Map},
    Archive, Serialize,
};
use randy_model::{
    gateway::{event::Event, payload::incoming::IntegrationCreate},
    guild::{
        GuildIntegration, GuildIntegrationType, IntegrationAccount, IntegrationApplication,
        IntegrationExpireBehavior,
    },
    id::Id,
};

use crate::pool;

#[tokio::test]
async fn test_integration() -> Result<(), CacheError> {
    struct Config;

    impl CacheConfig for Config {
        #[cfg(feature = "metrics")]
        const METRICS_INTERVAL_DURATION: Duration = Duration::from_secs(60);

        type Channel<'a> = Ignore;
        type CurrentUser<'a> = Ignore;
        type Emoji<'a> = Ignore;
        type Guild<'a> = Ignore;
        type Integration<'a> = CachedIntegration<'a>;
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

    #[derive(Archive, Serialize)]
    struct CachedIntegration<'a> {
        #[rkyv(with = IntegrationAccountRkyv)]
        account: &'a IntegrationAccount,
        #[rkyv(with = Map<RkyvAsU8>)]
        expire_behavior: Option<IntegrationExpireBehavior>,
        #[rkyv(with = GuildIntegrationTypeRkyv)]
        kind: &'a GuildIntegrationType,
        #[rkyv(with =  Map<InlineAsBox>)]
        scopes: Option<&'a [String]>,
    }

    impl<'a> ICachedIntegration<'a> for CachedIntegration<'a> {
        fn from_integration(integration: &'a GuildIntegration) -> Self {
            Self {
                account: &integration.account,
                expire_behavior: integration.expire_behavior,
                kind: &integration.kind,
                scopes: integration.scopes.as_deref(),
            }
        }
    }

    impl Cacheable for CachedIntegration<'_> {
        type Bytes = AlignedVec<8>;

        fn expire() -> Option<Duration> {
            None
        }

        fn serialize_one<E: Source>(&self) -> Result<Self::Bytes, E> {
            rkyv::api::high::to_bytes_in(self, AlignedVec::default())
        }

        // we don't update by deserializing so a `serialize_into` impl is not
        // necessary
    }

    impl PartialEq<GuildIntegration> for ArchivedCachedIntegration<'_> {
        fn eq(&self, other: &GuildIntegration) -> bool {
            self.account.id == other.account.id
                && self.account.name == other.account.name
                && self.expire_behavior == other.expire_behavior.map(u8::from)
                && self.kind.as_str() == Cow::from(other.kind.clone()).as_ref()
                && match (self.scopes.as_ref(), other.scopes.as_ref()) {
                    (None, None) => true,
                    (Some(this), Some(other)) => {
                        this.len() == other.len()
                            && this.iter().zip(other.iter()).all(|(a, b)| a == b)
                    }
                    _ => false,
                }
        }
    }

    impl Debug for ArchivedCachedIntegration<'_> {
        fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
            f.debug_struct("ArchivedCachedIntegration")
                .field("account", &self.account)
                .field("expire_behavior", &self.expire_behavior)
                .field("kind", &self.kind)
                .field("scopes", &self.scopes)
                .finish()
        }
    }

    let cache = RedisCache::<Config>::new_with_pool(pool()).await?;

    let expected = integration();

    let event = Event::IntegrationCreate(Box::new(IntegrationCreate(expected.clone())));
    cache.update(&event).await?;

    let integration = cache
        .integration(expected.guild_id.unwrap(), expected.id)
        .await?
        .expect("missing integration");

    assert_eq!(integration.deref(), &expected);

    let mut iter = cache
        .iter()
        .guild_integrations(expected.guild_id.unwrap())
        .await?;

    let integration_res = iter.next().expect("missing integration");

    #[cfg(feature = "bytecheck")]
    let integration = integration_res?;

    #[cfg(not(feature = "bytecheck"))]
    let integration = integration_res;

    assert_eq!(integration.deref(), &expected);

    assert!(iter.next().is_none());

    Ok(())
}

pub fn integration() -> GuildIntegration {
    GuildIntegration {
        account: IntegrationAccount {
            id: "account_id".to_owned(),
            name: "account_name".to_owned(),
        },
        application: Some(IntegrationApplication {
            bot: None,
            description: "application_description".to_owned(),
            icon: None,
            id: Id::new(891),
            name: "application_name".to_owned(),
        }),
        enable_emoticons: Some(true),
        enabled: Some(true),
        expire_behavior: Some(IntegrationExpireBehavior::RemoveRole),
        expire_grace_period: None,
        guild_id: Some(Id::new(332)),
        id: Id::new(433),
        kind: GuildIntegrationType::YouTube,
        name: "integration_name".to_owned(),
        revoked: None,
        role_id: Some(Id::new(112)),
        scopes: Some(vec!["scope1".to_owned(), "scope2".to_owned()]),
        subscriber_count: None,
        synced_at: None,
        syncing: None,
        user: None,
    }
}
