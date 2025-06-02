use randy_model::{
    gateway::payload::incoming::{GuildScheduledEventUserAdd, GuildScheduledEventUserRemove},
    guild::scheduled_event::GuildScheduledEvent,
    id::{
        marker::{GuildMarker, ScheduledEventMarker},
        Id,
    },
};
use rkyv::{api::high::to_bytes_in, rancor::Source, ser::writer::Buffer, Archived};
use tracing::{instrument, trace};

use crate::{
    cache::{
        meta::{atoi, HasArchived, IMeta, IMetaKey},
        pipe::Pipe,
    },
    config::{CacheConfig, Cacheable, ICachedScheduledEvent, SerializeMany},
    error::{
        MetaError, MetaErrorKind, SerializeError, SerializeErrorKind, UpdateError, UpdateErrorKind,
    },
    key::{name_id, RedisKey},
    redis::{Pipeline, RedisWrite, ToRedisArgs},
    rkyv_util::id::IdRkyv,
    util::BytesWrap,
    CacheResult, RedisCache,
};

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct ScheduledEventKey {
    pub id: Id<ScheduledEventMarker>,
}

impl RedisKey for ScheduledEventKey {
    const PREFIX: &'static [u8] = b"SCHEDULED_EVENT";
}

impl ToRedisArgs for ScheduledEventKey {
    fn write_redis_args<W>(&self, out: &mut W)
    where
        W: ?Sized + RedisWrite,
    {
        out.write_arg(name_id(Self::PREFIX, self.id).as_ref());
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct ScheduledEventMetaKey {
    pub id: Id<ScheduledEventMarker>,
}

impl RedisKey for ScheduledEventMetaKey {
    const PREFIX: &'static [u8] = b"SCHEDULED_EVENT_META";
}

impl ToRedisArgs for ScheduledEventMetaKey {
    fn write_redis_args<W>(&self, out: &mut W)
    where
        W: ?Sized + RedisWrite,
    {
        out.write_arg(name_id(Self::PREFIX, self.id).as_ref());
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct ScheduledEventsKey;

impl RedisKey for ScheduledEventsKey {
    const PREFIX: &'static [u8] = b"SCHEDULED_EVENTS";
}

impl ToRedisArgs for ScheduledEventsKey {
    fn write_redis_args<W>(&self, out: &mut W)
    where
        W: ?Sized + RedisWrite,
    {
        out.write_arg(Self::PREFIX);
    }
}

impl From<Id<ScheduledEventMarker>> for ScheduledEventKey {
    fn from(id: Id<ScheduledEventMarker>) -> Self {
        Self { id }
    }
}

impl<C: CacheConfig> RedisCache<C> {
    #[instrument(level = "trace", skip_all)]
    pub(crate) fn store_scheduled_event(
        &self,
        pipe: &mut Pipe<'_, C>,
        event: &GuildScheduledEvent,
    ) -> CacheResult<()> {
        if let Some(ref user) = event.creator {
            self.store_user(pipe, user)?;
        }

        if !C::ScheduledEvent::WANTED {
            return Ok(());
        }

        let event_id = event.id;
        let guild_id = event.guild_id;

        let key = ScheduledEventKey { id: event_id };

        let event = C::ScheduledEvent::from_scheduled_event(event);

        let bytes = event
            .serialize_one()
            .map_err(|e| SerializeError::new(e, SerializeErrorKind::ScheduledEvent))?;

        trace!(bytes = bytes.as_ref().len());

        pipe.set(key, bytes.as_ref(), C::ScheduledEvent::expire());

        let key = crate::cache::impls::guild::GuildScheduledEventsKey { id: guild_id };
        pipe.sadd(key, event_id.get());

        let key = ScheduledEventsKey;
        pipe.sadd(key, event_id.get());

        if C::ScheduledEvent::expire().is_some() {
            let key = ScheduledEventMetaKey { id: event_id };

            ScheduledEventMeta { guild: guild_id }
                .store(pipe, key)
                .map_err(|e| MetaError::new(e, MetaErrorKind::ScheduledEvent))?;
        }

        Ok(())
    }

    #[instrument(level = "trace", skip_all)]
    pub(crate) fn store_scheduled_events(
        &self,
        pipe: &mut Pipe<'_, C>,
        guild_id: Id<GuildMarker>,
        events: &[GuildScheduledEvent],
    ) -> CacheResult<()> {
        if !C::ScheduledEvent::WANTED {
            return Ok(());
        }

        let mut serializer = C::ScheduledEvent::serialize_many();

        let (event_entries, event_ids): (Vec<_>, Vec<_>) = events
            .iter()
            .map(|stage_instance| {
                let id = stage_instance.id;
                let key = ScheduledEventKey { id };
                let stage_instance = C::ScheduledEvent::from_scheduled_event(stage_instance);

                let bytes = serializer
                    .serialize_next(&stage_instance)
                    .map_err(|e| SerializeError::new(e, SerializeErrorKind::ScheduledEvent))?;

                trace!(bytes = bytes.as_ref().len());

                Ok(((key, BytesWrap(bytes)), id.get()))
            })
            .collect::<CacheResult<(Vec<(ScheduledEventKey, BytesWrap<_>)>, Vec<u64>)>>()?;

        if event_entries.is_empty() {
            return Ok(());
        }

        pipe.mset(&event_entries, C::ScheduledEvent::expire());

        let key = crate::cache::impls::guild::GuildScheduledEventsKey { id: guild_id };
        pipe.sadd(key, event_ids.as_slice());

        let key = ScheduledEventsKey;
        pipe.sadd(key, event_ids);

        if C::ScheduledEvent::expire().is_some() {
            events
                .iter()
                .try_for_each(|event| {
                    let key = ScheduledEventMetaKey { id: event.id };

                    ScheduledEventMeta { guild: guild_id }.store(pipe, key)
                })
                .map_err(|e| MetaError::new(e, MetaErrorKind::ScheduledEvent))?;
        }

        Ok(())
    }

    pub(crate) async fn store_scheduled_event_user_add(
        &self,
        pipe: &mut Pipe<'_, C>,
        event: &GuildScheduledEventUserAdd,
    ) -> CacheResult<()> {
        if !C::ScheduledEvent::WANTED {
            return Ok(());
        }

        let Some(update_fn) = C::ScheduledEvent::on_user_add_event() else {
            return Ok(());
        };

        let event_id = event.guild_scheduled_event_id;

        let key = ScheduledEventKey { id: event_id };

        let Some(mut archived) = pipe
            .get::<Archived<C::ScheduledEvent<'static>>>(key)
            .await?
        else {
            return Ok(());
        };

        update_fn(&mut archived, event)
            .map_err(|e| UpdateError::new(e, UpdateErrorKind::ScheduledEventUserAdd))?;

        let key = ScheduledEventKey { id: event_id };
        let bytes = archived.into_bytes();
        trace!(bytes = bytes.as_ref().len());
        pipe.set(key, &bytes, C::ScheduledEvent::expire());

        let key = ScheduledEventsKey;
        pipe.sadd(key, event_id.get());

        if C::ScheduledEvent::expire().is_some() {
            let meta = ScheduledEventMeta {
                guild: event.guild_id,
            };

            meta.store(pipe, ScheduledEventMetaKey { id: event_id })
                .map_err(|e| MetaError::new(e, MetaErrorKind::ScheduledEvent))?;
        }

        Ok(())
    }

    pub(crate) async fn store_scheduled_event_user_remove(
        &self,
        pipe: &mut Pipe<'_, C>,
        event: &GuildScheduledEventUserRemove,
    ) -> CacheResult<()> {
        if !C::ScheduledEvent::WANTED {
            return Ok(());
        }

        let Some(update_fn) = C::ScheduledEvent::on_user_remove_event() else {
            return Ok(());
        };

        let event_id = event.guild_scheduled_event_id;

        let key = ScheduledEventKey { id: event_id };

        let Some(mut archived) = pipe
            .get::<Archived<C::ScheduledEvent<'static>>>(key)
            .await?
        else {
            return Ok(());
        };

        update_fn(&mut archived, event)
            .map_err(|e| UpdateError::new(e, UpdateErrorKind::ScheduledEventUserAdd))?;

        let key = ScheduledEventKey { id: event_id };
        let bytes = archived.into_bytes();
        trace!(bytes = bytes.as_ref().len());
        pipe.set(key, &bytes, C::ScheduledEvent::expire());

        let key = ScheduledEventsKey;
        pipe.sadd(key, event_id.get());

        if C::ScheduledEvent::expire().is_some() {
            let meta = ScheduledEventMeta {
                guild: event.guild_id,
            };

            meta.store(pipe, ScheduledEventMetaKey { id: event_id })
                .map_err(|e| MetaError::new(e, MetaErrorKind::ScheduledEvent))?;
        }

        Ok(())
    }

    pub(crate) fn delete_scheduled_event(
        &self,
        pipe: &mut Pipe<'_, C>,
        event: &GuildScheduledEvent,
    ) -> CacheResult<()> {
        if let Some(ref user) = event.creator {
            self.store_user(pipe, user)?;
        }

        if !C::ScheduledEvent::WANTED {
            return Ok(());
        }

        let event_id = event.id;
        let guild_id = event.guild_id;

        let key = ScheduledEventKey { id: event_id };
        pipe.del(key);

        let key = crate::cache::impls::guild::GuildScheduledEventsKey { id: guild_id };
        pipe.srem(key, event_id.get());

        let key = ScheduledEventsKey;
        pipe.srem(key, event_id.get());

        if C::ScheduledEvent::expire().is_some() {
            let key = ScheduledEventMetaKey { id: event_id };
            pipe.del(key);
        }

        Ok(())
    }
}

impl IMetaKey for ScheduledEventMetaKey {
    fn parse<'a>(split: &mut impl Iterator<Item = &'a [u8]>) -> Option<Self> {
        split.next().and_then(atoi).map(|event| Self { id: event })
    }

    fn handle_expire(&self, pipe: &mut Pipeline) {
        let key = ScheduledEventsKey;
        pipe.srem(key, self.id.get()).ignore();
    }
}

impl HasArchived for ScheduledEventMetaKey {
    type Meta = ScheduledEventMeta;

    fn redis_key(&self) -> impl RedisKey {
        ScheduledEventMetaKey { id: self.id }
    }

    fn handle_archived(&self, pipe: &mut Pipeline, archived: &Archived<Self::Meta>) {
        let key = crate::cache::impls::guild::GuildScheduledEventsKey {
            id: archived.guild.into(),
        };
        pipe.srem(key, self.id.get());
    }
}

#[derive(rkyv::Archive, rkyv::Serialize)]
pub(crate) struct ScheduledEventMeta {
    #[rkyv(with = IdRkyv)]
    guild: Id<GuildMarker>,
}

impl IMeta<ScheduledEventMetaKey> for ScheduledEventMeta {
    type Bytes = [u8; 8];

    fn to_bytes<E: Source>(&self) -> Result<Self::Bytes, E> {
        let mut bytes = [0; 8];
        to_bytes_in(self, Buffer::from(&mut bytes))?;

        Ok(bytes)
    }
}
