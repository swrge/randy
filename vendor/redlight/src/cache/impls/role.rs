use randy_model::{
    guild::Role,
    id::{
        marker::{GuildMarker, RoleMarker},
        Id,
    },
};
use rkyv::{api::high::to_bytes_in, rancor::Source, Archived};
use tracing::{instrument, trace};

use crate::{
    cache::{
        meta::{atoi, HasArchived, IMeta, IMetaKey},
        pipe::Pipe,
    },
    config::{CacheConfig, Cacheable, ICachedRole, SerializeMany},
    error::{MetaError, MetaErrorKind, SerializeError, SerializeErrorKind},
    key::{name_id, RedisKey},
    redis::{Pipeline, RedisWrite, ToRedisArgs},
    rkyv_util::id::IdRkyv,
    util::BytesWrap,
    CacheResult, RedisCache,
};

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct RoleKey {
    pub id: Id<RoleMarker>,
}

impl RedisKey for RoleKey {
    const PREFIX: &'static [u8] = b"ROLE";
}

impl ToRedisArgs for RoleKey {
    fn write_redis_args<W>(&self, out: &mut W)
    where
        W: ?Sized + RedisWrite,
    {
        out.write_arg(name_id(Self::PREFIX, self.id).as_ref());
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct RoleMetaKey {
    pub id: Id<RoleMarker>,
}

impl RedisKey for RoleMetaKey {
    const PREFIX: &'static [u8] = b"ROLE_META";
}

impl ToRedisArgs for RoleMetaKey {
    fn write_redis_args<W>(&self, out: &mut W)
    where
        W: ?Sized + RedisWrite,
    {
        out.write_arg(name_id(Self::PREFIX, self.id).as_ref());
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct RolesKey;

impl RedisKey for RolesKey {
    const PREFIX: &'static [u8] = b"ROLES";
}

impl ToRedisArgs for RolesKey {
    fn write_redis_args<W>(&self, out: &mut W)
    where
        W: ?Sized + RedisWrite,
    {
        out.write_arg(Self::PREFIX);
    }
}

impl From<Id<RoleMarker>> for RoleKey {
    fn from(id: Id<RoleMarker>) -> Self {
        Self { id }
    }
}

impl<C: CacheConfig> RedisCache<C> {
    #[instrument(level = "trace", skip_all)]
    pub(crate) fn store_role(
        &self,
        pipe: &mut Pipe<'_, C>,
        guild_id: Id<GuildMarker>,
        role: &Role,
    ) -> CacheResult<()> {
        if !C::Role::WANTED {
            return Ok(());
        }

        let id = role.id;
        let key = RoleKey { id };
        let role = C::Role::from_role(role);

        let bytes = role
            .serialize_one()
            .map_err(|e| SerializeError::new(e, SerializeErrorKind::Role))?;

        trace!(bytes = bytes.as_ref().len());

        pipe.set(key, bytes.as_ref(), C::Role::expire());

        let key = crate::cache::impls::guild::GuildRolesKey { id: guild_id };
        pipe.sadd(key, id.get());

        let key = RolesKey;
        pipe.sadd(key, id.get());

        if C::Role::expire().is_some() {
            RoleMeta { guild: guild_id }
                .store(pipe, RoleMetaKey { id })
                .map_err(|e| MetaError::new(e, MetaErrorKind::Role))?;
        }

        Ok(())
    }

    #[instrument(level = "trace", skip_all)]
    pub(crate) fn store_roles<'a, I>(
        &self,
        pipe: &mut Pipe<'_, C>,
        guild_id: Id<GuildMarker>,
        roles: I,
    ) -> CacheResult<()>
    where
        I: IntoIterator<Item = &'a Role>,
    {
        if !C::Role::WANTED {
            return Ok(());
        }

        let with_expire = C::Role::expire().is_some();

        let mut serializer = C::Role::serialize_many();

        let (roles, role_ids) = roles
            .into_iter()
            .map(|role| {
                let id = role.id;
                let key = RoleKey { id };
                let cached = C::Role::from_role(role);

                if with_expire {
                    RoleMeta { guild: guild_id }
                        .store(pipe, RoleMetaKey { id })
                        .map_err(|e| MetaError::new(e, MetaErrorKind::Role))?;
                }

                let bytes = serializer
                    .serialize_next(&cached)
                    .map_err(|e| SerializeError::new(e, SerializeErrorKind::Role))?;

                trace!(bytes = bytes.as_ref().len());

                Ok(((key, BytesWrap(bytes)), id.get()))
            })
            .collect::<CacheResult<(Vec<(RoleKey, BytesWrap<_>)>, Vec<u64>)>>()?;

        if roles.is_empty() {
            return Ok(());
        }

        pipe.mset(&roles, C::Role::expire());

        let key = crate::cache::impls::guild::GuildRolesKey { id: guild_id };
        pipe.sadd(key, role_ids.as_slice());

        let key = RolesKey;
        pipe.sadd(key, role_ids);

        Ok(())
    }

    pub(crate) fn delete_role(
        &self,
        pipe: &mut Pipe<'_, C>,
        guild_id: Id<GuildMarker>,
        role_id: Id<RoleMarker>,
    ) {
        if !C::Role::WANTED {
            return;
        }

        let key = RoleKey { id: role_id };
        pipe.del(key);

        let key = crate::cache::impls::guild::GuildRolesKey { id: guild_id };
        pipe.srem(key, role_id.get());

        let key = RolesKey;
        pipe.srem(key, role_id.get());

        if C::Role::expire().is_some() {
            pipe.del(RoleMetaKey { id: role_id });
        }
    }
}

impl IMetaKey for RoleMetaKey {
    fn parse<'a>(split: &mut impl Iterator<Item = &'a [u8]>) -> Option<Self> {
        split.next().and_then(atoi).map(|role| Self { id: role })
    }

    fn handle_expire(&self, pipe: &mut Pipeline) {
        let key = RolesKey;
        pipe.srem(key, self.id.get()).ignore();
    }
}

impl HasArchived for RoleMetaKey {
    type Meta = RoleMeta;

    fn redis_key(&self) -> impl RedisKey {
        RoleMetaKey { id: self.id }
    }

    fn handle_archived(&self, pipe: &mut Pipeline, archived: &Archived<Self::Meta>) {
        let key = crate::cache::impls::guild::GuildRolesKey {
            id: archived.guild.into(),
        };
        pipe.srem(key, self.id.get());
    }
}

#[derive(rkyv::Archive, rkyv::Serialize)]
pub(crate) struct RoleMeta {
    #[rkyv(with = IdRkyv)]
    guild: Id<GuildMarker>,
}

impl IMeta<RoleMetaKey> for RoleMeta {
    type Bytes = [u8; 8];

    fn to_bytes<E: Source>(&self) -> Result<Self::Bytes, E> {
        let mut bytes = [0; 8];
        to_bytes_in(self, rkyv::ser::writer::Buffer::from(&mut bytes))?;

        Ok(bytes)
    }
}
