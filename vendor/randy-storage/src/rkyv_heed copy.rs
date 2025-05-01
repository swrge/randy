//use byteorder::NativeEndian;
//use heed::{types::*, BytesDecode, BytesEncode, Database, Env};
//use heed::byteorder::BigEndian;
//use heed::*;
//use util::AlignedVec;

use api::high::to_bytes_in;
use bytes::Bytes;
use libmdbx::orm::{table, table_info, DatabaseChart, Decodable, Encodable};
use libmdbx::*;
use orm::Transaction;
use randy_model::channel::{Channel, Message};
use randy_model::guild::{Guild, Member};
use randy_model::id::marker::*;
use randy_model::id::*;
use rkyv::string::{repr::*, ArchivedString, StringResolver};
use rkyv::*;
use std::borrow::Cow;
use std::num::NonZeroU64;
use std::path::Path;
use std::str::Bytes;
use std::sync::Once;
use util::AlignedVec;

#[allow(clippy::struct_excessive_bools)]
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize, Archive)]
#[rkyv(archived = CachedEmoji)]
struct Emoji {
    pub(crate) animated: bool,
    pub(crate) available: bool,
    pub(crate) id: Id<EmojiMarker>,
    pub(crate) managed: bool,
    pub(crate) name: String,
    pub(crate) require_colons: bool,
    pub(crate) roles: Vec<Id<RoleMarker>>,
    pub(crate) user_id: Option<Id<UserMarker>>,
}

struct Cacheable<T>(T);

//type NEU64 = U64<NativeEndian>;
//type Emojis = Database<NEU64, Bytes>;
//pub struct Set<K>(Database<K, Unit>);

pub struct RandyStorage {
    db: Database<WriteMap>,
    rw: Transaction, //channels: Channels,
                     //messages: Messages,
                     //members: Members,
}

impl RandyStorage {
    // Initial size of 10MB
    const INITIAL_MAP_SIZE: usize = 10 * 1024 * 1024;

    // Growth factor when resizing (doubles the size)
    const GROWTH_FACTOR: usize = 2;

    fn new(path: impl AsRef<Path>) -> Result<Self> {
        std::fs::create_dir_all(&path).expect("bad directory");

        let db = Database::<WriteMap>::open_with_options(
            path,
            DatabaseOptions {
                // todo
                permissions: Some(0o644),

                // -- mdbx_env_set_maxreaders --
                // Set the maximum number of threads/reader slots  for all
                // processes interacts with the database. (default :100 for 4K system page size)
                max_readers: Some(100),

                // -- mdbx_env_set_maxdbs --
                // Set the maximum number of named databases for the environment.
                max_tables: Some(26),

                // -- MDBX_opt_rp_augment_limit --
                // Controls the in-process limit to grow a list of reclaimed/recycled
                // page's numbers for finding a sequence of contiguous pages for large data items.
                // Default is 262144.
                rp_augment_limit: Some(262_144),

                // -- MDBX_opt_loose_limit --
                // Controls the in-process limit to grow a cache of dirty pages for
                // reuse in the current transaction.
                // Should be in the range 0..255, default is 64.
                loose_limit: Some(64),

                // -- MDBX_opt_dp_reserve_limit --
                // Controls the in-process limit of a pre-allocated memory items for
                // dirty pages. Default is 1024.
                dp_reserve_limit: Some(2046),

                // -- MDBX_opt_txn_dp_limit --
                // Controls the in-process limit of dirty pages for a write transaction.
                // Default is 65536
                txn_dp_limit: Some(65536),

                // -- MDBX_opt_spill_max_denominator --
                // Controls the in-process how maximal part of the dirty pages may be spilled when necessary.
                // Should be in the range 0..255, where zero means no limit, i.e. all dirty pages could be spilled.
                // Default is 8
                spill_max_denominator: Some(8),

                // -- MDBX_opt_spill_min_denominator --
                // Controls the in-process how minimal part of the dirty pages should be spilled when necessary.
                // Should be in range 0..255, where zero means no restriction at the bottom.
                // Default is 8
                spill_min_denominator: Some(8),

                // -- MDBX_PAGESIZE --
                // Should be in range 256..65536. Default is automatic
                page_size: Some(PageSize::MinimalAcceptable),

                // -- MDBX_NOSUBDIR --
                // No environment directory
                // By default, MDBX creates its environment in a directory whose pathname is given in path,
                // and creates its data and lock files under that directory. With this option, path is used
                // as-is for the database main data file. The database lock file is the path with "-lck" appended.
                no_sub_dir: true,

                // -- MDBX_EXCLUSIVE --
                // Open environment in exclusive/monopolistic mode.
                // MDBX_EXCLUSIVE flag can be used as a replacement for MDB_NOLOCK, which don't supported by MDBX.
                // In this way, you can get the minimal overhead, but with the correct multi-process and multi-thread locking.
                exclusive: false,

                // -- MDBX_ACCEDE --
                // Using database/environment which already opened by another process(es).
                // The MDBX_ACCEDE flag is useful to avoid MDBX_INCOMPATIBLE error while opening the
                // database/environment which is already used by another process(es) with unknown mode/flags.
                accede: true,

                // -- MDBX_SYNC_DURABLE  --
                // The default A.C.I.D mode.
                // Metadata is written and flushed to disk after data is committed which guarantees
                // the integrity of the database in the event of a crash at any time.
                //
                //
                // -- MDBX_NOMETASYNC --
                // Don't sync the meta-page after commit.
                //
                // -- MDBX_SAFE_NOSYNC --
                // Don't sync anything but keep previous steady commits.
                // In case of a system/app crash, it will lose the last transaction
                // but it won't corrupt the whole DB.
                //
                // -- MDBX_UTTERLY_NOSYNC --
                // Don't sync anything and wipe previous steady commits.
                // This is unsafe because a system/app crash will corrupt the whole DB.
                // But you get ~100x more write performance in return. Choices, choices...
                mode: Mode::ReadWrite(ReadWriteOptions {
                    sync_mode: SyncMode::Durable,
                    ..Default::default()
                }),

                // -- MDBX_NORDAHEAD --
                // Don't do readahead.
                // Turn off readahead. Most operating systems perform readahead on read requests by default.
                // This option turns it off if the OS supports it. Turning it off may help random read
                // performance when the DB is larger than RAM and system RAM is full.
                no_rdahead: false,

                // -- MDBX_NOMEMINIT --
                // Don't initialize malloc'ed memory before writing to datafile.
                // By default, memory for pages written to the data file is obtained using malloc.
                // While these pages may be reused in subsequent transactions, freshly malloc'ed
                // pages will be initialized to zeroes before use. This avoids persisting leftover
                // data from other code (that used the heap and subsequently freed the memory)
                // into the data file. In other words, to trade safety for performance, again.
                no_meminit: false,

                // -- MDBX_COALESCE --
                // Aims to coalesce GCed items.
                // With MDBX_COALESCE flag MDBX will aims to coalesce memory while recycling
                // Garbage Collected items.
                coalesce: true,

                // -- MDBX_LIFORECLAIM --
                // LIFO policy for recycling a Garbage Collection items.
                // MDBX_LIFORECLAIM flag turns on LIFO policy for recycling a Garbage Collection items, instead of FIFO by default.
                // On systems with a disk write-back cache, this can significantly increase write performance, up to several times
                // in a best case scenario.
                liforeclaim: true,
            },
        )?;

        Ok(Self { db })
    }

    fn insert_emoji<'a>(&'a self, wtxn: &'a mut RwTxn, emoji: &Emoji) -> Result<()> {
        let bytes = rkyv::to_bytes::<rancor::Error>(emoji).unwrap();
        self.emojis.put(wtxn, &emoji.id.get(), &bytes)?;
        Ok(())
    }
    //
    //    fn get_emoji<'a>(
    //        &'a self,
    //        rtxn: &'a heed::RoTxn,
    //        id: Id<EmojiMarker>,
    //    ) -> Result<Option<&'a CachedEmoji>> {
    //        if let Some(bytes) = self.emojis.get(&rtxn, &id.get())? {
    //            let archive = access::<CachedEmoji, rancor::Error>(bytes).unwrap();
    //            Ok(Some(archive))
    //        } else {
    //            Ok(None)
    //        }
    //    }
    //
    //    fn delete_emoji<'a>(&'a self, wtxn: &'a mut RwTxn, id: Id<EmojiMarker>) -> Result<()> {
    //        self.emojis.delete(wtxn, &id.get())?;
    //        Ok(())
    //    }
    //
    //    fn iter_emojis<'a>(&'a self, rtxn: &'a RoTxn) -> Result<impl Iterator<Item = &'a CachedEmoji>> {
    //        let process =
    //            |e: std::result::Result<(u64, &'a [u8]), heed::Error>| -> Option<&CachedEmoji> {
    //                let (_, bytes) = e.ok()?;
    //                access::<CachedEmoji, rancor::Error>(bytes).ok()
    //            };
    //
    //        let iter = self.emojis.iter(&rtxn)?.filter_map(process);
    //
    //        Ok(iter)
    //    }
}

//impl RandyStorage {
//    fn new(path: &str) -> Self {
//        #[allow(unsafe_code)]
//        let env = unsafe {
//            heed::EnvOpenOptions::new()
//                .map_size(16384) // one page
//                .open(&path)
//                .unwrap()
//        };
//        Self { env }
//    }
//}
//channels: Database<Id<ChannelMarker>, Channel>,
//channel_messages: Database<Id<ChannelMarker>, VecDeque<Id<MessageMarker>>>,
//emojis: Database<Id<EmojiMarker>, GuildResource<Emoji>>,
//guilds: Database<Id<GuildMarker>, Guild>,
//guild_channels: Database<Id<GuildMarker>, HashSet<Id<ChannelMarker>>>,
//guild_emojis: Database<Id<GuildMarker>, HashSet<Id<EmojiMarker>>>,
//guild_integrations: Database<Id<GuildMarker>, HashSet<Id<IntegrationMarker>>>,
//guild_members: Database<Id<GuildMarker>, HashSet<Id<UserMarker>>>,
//guild_presences: Database<Id<GuildMarker>, HashSet<Id<UserMarker>>>,
//guild_roles: Database<Id<GuildMarker>, HashSet<Id<RoleMarker>>>,
//guild_scheduled_events: Database<Id<GuildMarker>, HashSet<Id<ScheduledEventMarker>>>,
//guild_stage_instances: Database<Id<GuildMarker>, HashSet<Id<StageMarker>>>,
//guild_stickers: Database<Id<GuildMarker>, HashSet<Id<StickerMarker>>>,
//integrations:
//    Database<(Id<GuildMarker>, Id<IntegrationMarker>), GuildResource<GuildIntegration>>,
//members: Database<(Id<GuildMarker>, Id<UserMarker>), Member>,
//messages: Database<Id<MessageMarker>, Message>,
//presences: Database<(Id<GuildMarker>, Id<UserMarker>), Presence>,
//roles: Database<Id<RoleMarker>, GuildResource<Role>>,
//scheduled_events: Database<Id<ScheduledEventMarker>, GuildResource<GuildScheduledEvent>>,
//stage_instances: Database<Id<StageMarker>, GuildResource<StageInstance>>,
//stickers: Database<Id<StickerMarker>, GuildResource<Sticker>>,
//unavailable_guilds: Database<Id<GuildMarker>, rkyv_heed::types::Unit>,
//users: Database<Id<UserMarker>, User>,
//user_guilds: Database<Id<UserMarker>, HashSet<Id<GuildMarker>>>,
///// Mapping of channels and the users currently connected.
