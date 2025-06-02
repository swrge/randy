use randy_model::{
    channel::StageInstance,
    id::{
        marker::{GuildMarker, StageMarker},
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
    config::{CacheConfig, Cacheable, ICachedStageInstance, SerializeMany},
    error::{MetaError, MetaErrorKind, SerializeError, SerializeErrorKind},
    key::{name_id, RedisKey},
    redis::{Pipeline, RedisWrite, ToRedisArgs},
    rkyv_util::id::IdRkyv,
    util::BytesWrap,
    CacheResult, RedisCache,
};

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct StageInstanceKey {
    pub id: Id<StageMarker>,
}

impl RedisKey for StageInstanceKey {
    const PREFIX: &'static [u8] = b"STAGE_INSTANCE";
}

impl ToRedisArgs for StageInstanceKey {
    fn write_redis_args<W>(&self, out: &mut W)
    where
        W: ?Sized + RedisWrite,
    {
        out.write_arg(name_id(Self::PREFIX, self.id).as_ref());
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct StageInstanceMetaKey {
    pub id: Id<StageMarker>,
}

impl RedisKey for StageInstanceMetaKey {
    const PREFIX: &'static [u8] = b"STAGE_INSTANCE_META";
}

impl ToRedisArgs for StageInstanceMetaKey {
    fn write_redis_args<W>(&self, out: &mut W)
    where
        W: ?Sized + RedisWrite,
    {
        out.write_arg(name_id(Self::PREFIX, self.id).as_ref());
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct StageInstancesKey;

impl RedisKey for StageInstancesKey {
    const PREFIX: &'static [u8] = b"STAGE_INSTANCES";
}

impl ToRedisArgs for StageInstancesKey {
    fn write_redis_args<W>(&self, out: &mut W)
    where
        W: ?Sized + RedisWrite,
    {
        out.write_arg(Self::PREFIX);
    }
}

impl From<Id<StageMarker>> for StageInstanceKey {
    fn from(id: Id<StageMarker>) -> Self {
        Self { id }
    }
}

impl<C: CacheConfig> RedisCache<C> {
    #[instrument(level = "trace", skip_all)]
    pub(crate) fn store_stage_instance(
        &self,
        pipe: &mut Pipe<'_, C>,
        stage_instance: &StageInstance,
    ) -> CacheResult<()> {
        if !C::StageInstance::WANTED {
            return Ok(());
        }

        let stage_instance_id = stage_instance.id;
        let guild_id = stage_instance.guild_id;
        let key = StageInstanceKey {
            id: stage_instance_id,
        };
        let stage_instance = C::StageInstance::from_stage_instance(stage_instance);

        let bytes = stage_instance
            .serialize_one()
            .map_err(|e| SerializeError::new(e, SerializeErrorKind::StageInstance))?;

        trace!(bytes = bytes.as_ref().len());

        pipe.set(key, bytes.as_ref(), C::StageInstance::expire());

        let key = crate::cache::impls::guild::GuildStageInstancesKey { id: guild_id };
        pipe.sadd(key, stage_instance_id.get());

        let key = StageInstancesKey;
        pipe.sadd(key, stage_instance_id.get());

        if C::StageInstance::expire().is_some() {
            let key = StageInstanceMetaKey {
                id: stage_instance_id,
            };

            StageInstanceMeta { guild: guild_id }
                .store(pipe, key)
                .map_err(|e| MetaError::new(e, MetaErrorKind::StageInstance))?;
        }

        Ok(())
    }

    #[instrument(level = "trace", skip_all)]
    pub(crate) fn store_stage_instances(
        &self,
        pipe: &mut Pipe<'_, C>,
        guild_id: Id<GuildMarker>,
        stage_instances: &[StageInstance],
    ) -> CacheResult<()> {
        if !C::StageInstance::WANTED {
            return Ok(());
        }

        let mut serializer = C::StageInstance::serialize_many();

        let (stage_instance_entries, stage_instance_ids): (Vec<_>, Vec<_>) = stage_instances
            .iter()
            .map(|stage_instance| {
                let id = stage_instance.id;
                let key = StageInstanceKey { id };
                let stage_instance = C::StageInstance::from_stage_instance(stage_instance);

                let bytes = serializer
                    .serialize_next(&stage_instance)
                    .map_err(|e| SerializeError::new(e, SerializeErrorKind::StageInstance))?;

                trace!(bytes = bytes.as_ref().len());

                Ok(((key, BytesWrap(bytes)), id.get()))
            })
            .collect::<CacheResult<(Vec<(StageInstanceKey, BytesWrap<_>)>, Vec<u64>)>>()?;

        if stage_instance_entries.is_empty() {
            return Ok(());
        }

        pipe.mset(&stage_instance_entries, C::StageInstance::expire());

        let key = crate::cache::impls::guild::GuildStageInstancesKey { id: guild_id };
        pipe.sadd(key, stage_instance_ids.as_slice());

        let key = StageInstancesKey;
        pipe.sadd(key, stage_instance_ids);

        if C::StageInstance::expire().is_some() {
            stage_instances
                .iter()
                .try_for_each(|stage_instance| {
                    let key = StageInstanceMetaKey {
                        id: stage_instance.id,
                    };

                    StageInstanceMeta { guild: guild_id }.store(pipe, key)
                })
                .map_err(|e| MetaError::new(e, MetaErrorKind::StageInstance))?;
        }

        Ok(())
    }

    pub(crate) fn delete_stage_instance(
        &self,
        pipe: &mut Pipe<'_, C>,
        guild_id: Id<GuildMarker>,
        stage_instance_id: Id<StageMarker>,
    ) {
        if !C::StageInstance::WANTED {
            return;
        }

        let key = StageInstanceKey {
            id: stage_instance_id,
        };
        pipe.del(key);

        let key = crate::cache::impls::guild::GuildStageInstancesKey { id: guild_id };
        pipe.srem(key, stage_instance_id.get());

        let key = StageInstancesKey;
        pipe.srem(key, stage_instance_id.get());

        if C::StageInstance::expire().is_some() {
            let key = StageInstanceMetaKey {
                id: stage_instance_id,
            };
            pipe.del(key);
        }
    }
}

impl IMetaKey for StageInstanceMetaKey {
    fn parse<'a>(split: &mut impl Iterator<Item = &'a [u8]>) -> Option<Self> {
        split.next().and_then(atoi).map(|stage| Self { id: stage })
    }

    fn handle_expire(&self, pipe: &mut Pipeline) {
        let key = StageInstancesKey;
        pipe.srem(key, self.id.get()).ignore();
    }
}

impl HasArchived for StageInstanceMetaKey {
    type Meta = StageInstanceMeta;

    fn redis_key(&self) -> impl RedisKey {
        StageInstanceMetaKey { id: self.id }
    }

    fn handle_archived(&self, pipe: &mut Pipeline, archived: &Archived<Self::Meta>) {
        let key = crate::cache::impls::guild::GuildStageInstancesKey {
            id: archived.guild.into(),
        };
        pipe.srem(key, self.id.get());
    }
}

#[derive(rkyv::Archive, rkyv::Serialize)]
pub(crate) struct StageInstanceMeta {
    #[rkyv(with = IdRkyv)]
    guild: Id<GuildMarker>,
}

impl IMeta<StageInstanceMetaKey> for StageInstanceMeta {
    type Bytes = [u8; 8];

    fn to_bytes<E: Source>(&self) -> Result<Self::Bytes, E> {
        let mut bytes = [0; 8];
        to_bytes_in(self, Buffer::from(&mut bytes))?;

        Ok(bytes)
    }
}
