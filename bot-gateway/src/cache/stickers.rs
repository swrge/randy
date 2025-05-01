use std::{
    fmt::{Debug, Formatter, Result as FmtResult},
    ops::Deref,
    time::Duration,
};

use randy_model::{
    channel::message::{
        Sticker,
        sticker::{StickerFormatType, StickerType},
    },
    gateway::{event::Event, payload::incoming::GuildStickersUpdate},
    id::Id,
};
use redlight::{
    RedisCache,
    config::{CacheConfig, Cacheable, ICachedSticker, Ignore},
    error::CacheError,
    rkyv_util::util::RkyvAsU8,
};
use rkyv::{
    Archive, Serialize,
    rancor::Source,
    util::AlignedVec,
    with::{InlineAsBox, Map},
};

#[derive(Archive, Serialize)]
pub struct CachedSticker<'a> {
    #[rkyv(with = Map<InlineAsBox>)]
    description: Option<&'a str>,
    #[rkyv(with = RkyvAsU8)]
    format_type: StickerFormatType,
    #[rkyv(with = RkyvAsU8)]
    kind: StickerType,
}

impl<'a> ICachedSticker<'a> for CachedSticker<'a> {
    fn from_sticker(sticker: &'a Sticker) -> Self {
        Self {
            description: sticker.description.as_deref(),
            format_type: sticker.format_type,
            kind: sticker.kind,
        }
    }
}

impl Cacheable for CachedSticker<'_> {
    type Bytes = AlignedVec;

    fn expire() -> Option<Duration> {
        None
    }

    fn serialize_one<E: Source>(&self) -> Result<Self::Bytes, E> {
        rkyv::to_bytes(self)
    }

    // we don't update by deserializing so a `serialize_into` impl is not
    // necessary
}

impl PartialEq<Sticker> for ArchivedCachedSticker<'_> {
    fn eq(&self, other: &Sticker) -> bool {
        self.description.as_deref() == other.description.as_deref()
            && self.format_type == u8::from(other.format_type)
            && self.kind == u8::from(other.kind)
    }
}

impl Debug for ArchivedCachedSticker<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        f.debug_struct("ArchivedCachedSticker")
            .field("description", &self.description)
            .field("format_type", &StickerFormatType::from(self.format_type))
            .field("kind", &StickerType::from(self.kind))
            .finish()
    }
}
