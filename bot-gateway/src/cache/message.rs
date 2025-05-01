use std::{
    fmt::{Debug, Formatter, Result as FmtResult},
    ops::Deref,
    time::Duration,
};

use randy_model::{
    channel::{
        ChannelMention, ChannelType, Message,
        message::{
            EmojiReactionType, Mention, MessageActivity, MessageActivityType, MessageFlags,
            MessageType, Reaction, ReactionCountDetails, RoleSubscriptionData,
            sticker::{MessageSticker, StickerFormatType},
        },
    },
    gateway::{
        event::Event,
        payload::incoming::{MessageCreate, MessageUpdate},
    },
    id::Id,
    user::UserFlags,
    util::Timestamp,
};
use redlight::{
    CachedArchive, RedisCache,
    config::{CacheConfig, Cacheable, ICachedMessage, Ignore, ReactionEvent},
    error::CacheError,
    rkyv_util::{flags::BitflagsRkyv, util::RkyvAsU8},
};
use rkyv::{
    Archive, Archived, Serialize, rancor::Source, ser::writer::Buffer, util::Align, with::Map,
};

#[derive(Archive, Serialize)]
pub struct CachedMessage {
    #[rkyv(with = Map<BitflagsRkyv>)]
    flags: Option<MessageFlags>,
    #[rkyv(with = RkyvAsU8)]
    kind: MessageType,
    timestamp: i64,
}

impl<'a> ICachedMessage<'a> for CachedMessage {
    fn from_message(message: &'a Message) -> Self {
        Self {
            flags: message.flags,
            kind: message.kind,
            timestamp: message.timestamp.as_micros(),
        }
    }

    fn on_message_update<E: Source>()
    -> Option<fn(&mut CachedArchive<Archived<Self>>, &MessageUpdate) -> Result<(), E>> {
        Some(|archived, update| {
            archived.update_archive(|sealed| {
                rkyv::munge::munge! {
                    let ArchivedCachedMessage { mut kind, mut timestamp, .. } = sealed
                };

                *kind = u8::from(update.kind);
                *timestamp = update.timestamp.as_micros().into();
            });

            Ok(())
        })
    }

    fn on_reaction_event<E: Source>()
    -> Option<fn(&mut CachedArchive<Archived<Self>>, ReactionEvent<'_>) -> Result<(), E>> {
        None
    }
}

impl Cacheable for CachedMessage {
    type Bytes = [u8; 32];

    fn expire() -> Option<Duration> {
        None
    }

    fn serialize_one<E: Source>(&self) -> Result<Self::Bytes, E> {
        let mut bytes = Align([0_u8; 32]);
        rkyv::api::high::to_bytes_in(self, Buffer::from(&mut *bytes))?;

        Ok(bytes.0)
    }
}

impl PartialEq<Message> for ArchivedCachedMessage {
    fn eq(&self, other: &Message) -> bool {
        self.flags == other.flags && self.kind == u8::from(other.kind)
    }
}

impl Debug for ArchivedCachedMessage {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        f.debug_struct("ArchivedCachedMessage")
            .field("flags", &self.flags)
            .field("kind", &self.kind)
            .finish()
    }
}
