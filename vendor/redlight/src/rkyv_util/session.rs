use std::{collections::HashMap, hash::BuildHasher};

use rkyv::{
    munge::munge,
    rancor::Fallible,
    ser::{Allocator, Writer},
    vec::{ArchivedVec, VecResolver},
    with::{ArchiveWith, DeserializeWith, InlineAsBox, SerializeWith},
    Archive, Archived, Deserialize, Place, Portable, Resolver, Serialize,
};
use randy_gateway::Session;

type ShardId = u32;
type Sessions<H> = HashMap<ShardId, Session, H>;

pub struct SessionsRkyv;

pub type ArchivedSessions = ArchivedVec<ArchivedSessionEntry>;

impl<H> ArchiveWith<Sessions<H>> for SessionsRkyv {
    type Archived = ArchivedSessions;
    type Resolver = VecResolver;

    fn resolve_with(sessions: &Sessions<H>, resolver: Self::Resolver, out: Place<Self::Archived>) {
        ArchivedVec::resolve_from_len(sessions.len(), resolver, out);
    }
}

impl<H, S> SerializeWith<Sessions<H>, S> for SessionsRkyv
where
    S: Fallible + Writer + Allocator + ?Sized,
{
    fn serialize_with(sessions: &Sessions<H>, s: &mut S) -> Result<Self::Resolver, S::Error> {
        let iter = sessions.iter().map(|(key, value)| SessionEntry {
            shard_id: *key,
            session_id: value.id(),
            session_sequence: value.sequence(),
        });

        ArchivedVec::serialize_from_iter(iter, s)
    }
}

impl<H, D> DeserializeWith<ArchivedSessions, HashMap<ShardId, Session, H>, D> for SessionsRkyv
where
    Archived<ShardId>: Deserialize<ShardId, D>,
    D: Fallible + ?Sized,
    H: Default + BuildHasher,
{
    fn deserialize_with(
        archived: &ArchivedSessions,
        _: &mut D,
    ) -> Result<HashMap<ShardId, Session, H>, D::Error> {
        let mut result = HashMap::with_capacity_and_hasher(archived.len(), H::default());

        for entry in archived.iter() {
            let shard_id = entry.shard_id.to_native();
            let session_id = entry.session_id.as_ref().to_owned();
            let session_sequence = entry.session_sequence.to_native();
            result.insert(shard_id, Session::new(session_sequence, session_id));
        }

        Ok(result)
    }
}

struct SessionEntry<'a> {
    shard_id: ShardId,
    session_id: &'a str,
    session_sequence: u64,
}

#[derive(Portable)]
#[cfg_attr(
    feature = "bytecheck",
    derive(rkyv::bytecheck::CheckBytes),
    bytecheck(crate = rkyv::bytecheck),
)]
#[repr(C)]
pub struct ArchivedSessionEntry {
    pub shard_id: Archived<ShardId>,
    pub session_id: Archived<Box<str>>,
    pub session_sequence: Archived<u64>,
}

struct SessionEntryResolver {
    shard_id: Resolver<ShardId>,
    session_id: Resolver<Box<str>>,
    session_sequence: Resolver<u64>,
}

impl Archive for SessionEntry<'_> {
    type Archived = ArchivedSessionEntry;
    type Resolver = SessionEntryResolver;

    fn resolve(&self, resolver: Self::Resolver, out: Place<Self::Archived>) {
        munge!(
            let ArchivedSessionEntry {
                shard_id,
                session_id,
                session_sequence
            } = out
        );
        self.shard_id.resolve(resolver.shard_id, shard_id);
        InlineAsBox::resolve_with(&self.session_id, resolver.session_id, session_id);
        self.session_sequence
            .resolve(resolver.session_sequence, session_sequence);
    }
}

impl<S: Fallible + Writer + ?Sized> Serialize<S> for SessionEntry<'_> {
    fn serialize(&self, serializer: &mut S) -> Result<Self::Resolver, <S as Fallible>::Error> {
        Ok(SessionEntryResolver {
            shard_id: self.shard_id.serialize(serializer)?,
            session_id: InlineAsBox::serialize_with(&self.session_id, serializer)?,
            session_sequence: self.session_sequence.serialize(serializer)?,
        })
    }
}

#[cfg(test)]
mod tests {
    use std::{hash::RandomState, iter};

    use rkyv::{rancor::Error, with::With};

    use super::*;

    fn session() -> Session {
        Session::new(123, "session_id".to_owned())
    }

    #[test]
    fn test_rkyv_session() -> Result<(), Error> {
        let session = session();

        let entry = SessionEntry {
            shard_id: 123,
            session_id: session.id(),
            session_sequence: session.sequence(),
        };

        let bytes = rkyv::to_bytes(&entry)?;

        #[cfg(not(feature = "bytecheck"))]
        let archived: &ArchivedSessionEntry = unsafe { rkyv::access_unchecked(&bytes) };

        #[cfg(feature = "bytecheck")]
        let archived: &ArchivedSessionEntry = rkyv::access(&bytes)?;

        let deserialized = Session::new(
            archived.session_sequence.into(),
            archived.session_id.as_ref().to_owned(),
        );

        assert_eq!(session, deserialized);

        Ok(())
    }

    #[test]
    fn test_rkyv_sessions() -> Result<(), Error> {
        let sessions: HashMap<_, _> = (0..).zip(iter::repeat(session()).take(10)).collect();
        let bytes = rkyv::to_bytes(With::<_, SessionsRkyv>::cast(&sessions))?;

        #[cfg(not(feature = "bytecheck"))]
        let archived: &ArchivedSessions = unsafe { rkyv::access_unchecked(&bytes) };

        #[cfg(feature = "bytecheck")]
        let archived: &ArchivedSessions = rkyv::access(&bytes)?;

        let deserialized: Sessions<RandomState> =
            rkyv::deserialize(With::<_, SessionsRkyv>::cast(archived))?;

        assert_eq!(sessions, deserialized);

        Ok(())
    }
}
