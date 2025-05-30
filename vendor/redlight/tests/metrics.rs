#![cfg(feature = "metrics")]

use std::{
    fmt::Write,
    ops::DerefMut,
    sync::{Arc, Mutex},
    time::Duration,
};

#[cfg(feature = "bb8")]
use bb8_redis::redis;
#[cfg(all(not(feature = "bb8"), feature = "deadpool"))]
use deadpool_redis::redis;
use metrics::{
    Counter, Gauge, GaugeFn, Histogram, Key, KeyName, Metadata, Recorder, SharedString, Unit,
};
use redis::Cmd;
use redlight::{
    config::{CacheConfig, Cacheable, ICachedChannel, ICachedSticker, Ignore},
    error::CacheError,
    CachedArchive, RedisCache,
};
use rkyv::{rancor::Source, Archive, Archived, Serialize};
use randy_model::{
    channel::{message::Sticker, Channel},
    gateway::{
        event::Event,
        payload::incoming::{ChannelCreate, ChannelPinsUpdate, GuildStickersUpdate},
    },
};

use crate::{
    events::{channel::text_channel, sticker::stickers},
    pool,
};

#[tokio::test]
async fn test_metrics() -> Result<(), CacheError> {
    struct Config;

    impl CacheConfig for Config {
        const METRICS_INTERVAL_DURATION: Duration = Duration::from_secs(2);

        type Channel<'a> = CachedChannel;
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
        type Sticker<'a> = CachedSticker;
        type User<'a> = Ignore;
        type VoiceState<'a> = Ignore;
    }

    #[derive(Archive, Serialize)]
    struct CachedChannel;

    impl<'a> ICachedChannel<'a> for CachedChannel {
        fn from_channel(_: &'a Channel) -> Self {
            Self
        }

        fn on_pins_update<E: Source>(
        ) -> Option<fn(&mut CachedArchive<Archived<Self>>, &ChannelPinsUpdate) -> Result<(), E>>
        {
            None
        }
    }

    impl Cacheable for CachedChannel {
        type Bytes = [u8; 0];

        fn expire() -> Option<Duration> {
            None
        }

        fn serialize_one<E: Source>(&self) -> Result<Self::Bytes, E> {
            Ok([])
        }
    }

    #[derive(Archive, Serialize)]
    struct CachedSticker;

    impl<'a> ICachedSticker<'a> for CachedSticker {
        fn from_sticker(_: &'a Sticker) -> Self {
            Self
        }
    }

    impl Cacheable for CachedSticker {
        type Bytes = [u8; 0];

        fn expire() -> Option<Duration> {
            None
        }

        fn serialize_one<E: Source>(&self) -> Result<Self::Bytes, E> {
            Ok([])
        }
    }

    struct GaugeHandle {
        value: Mutex<f64>,
    }

    impl GaugeFn for GaugeHandle {
        fn increment(&self, value: f64) {
            self.set(value);
        }

        fn decrement(&self, value: f64) {
            self.set(value);
        }

        fn set(&self, value: f64) {
            *self.value.lock().unwrap() = value;
        }
    }

    #[derive(Clone, Default)]
    struct MetricRecorder {
        inner: Arc<InnerRecorder>,
    }

    #[derive(Default)]
    struct InnerRecorder {
        gauges: Mutex<Vec<(Key, Arc<GaugeHandle>)>>,
    }

    impl MetricRecorder {
        fn render(&self) -> String {
            let mut res = String::new();
            let gauges = self.inner.gauges.lock().unwrap();

            let mut iter = gauges.iter();
            let last = iter.next_back();

            if let Some((key, gauge)) = last {
                for (key, gauge) in iter {
                    let _ = writeln!(res, "{}: {}", key.name(), gauge.value.lock().unwrap());
                }

                let _ = write!(res, "{}: {}", key.name(), gauge.value.lock().unwrap());
            }

            res
        }
    }

    #[rustfmt::skip]
    impl Recorder for MetricRecorder {
        fn register_gauge(&self, key: &Key, _: &Metadata) -> Gauge {
            let mut gauges = self.inner.gauges.lock().unwrap();

            let new_gauge = match gauges.iter().find(|(entry, _)| entry == key) {
                Some((_, gauge)) => gauge,
                None => {
                    let gauge = Arc::new(GaugeHandle { value: Mutex::new(0.0) });
                    gauges.push((key.to_owned(), gauge));
                    let (_, new_gauge) = &gauges[gauges.len() - 1];

                    new_gauge
                },
            };

            Gauge::from_arc(Arc::clone(new_gauge))
        }

        fn describe_counter(&self, _: KeyName, _: Option<Unit>, _: SharedString) {}
        fn describe_gauge(&self, _: KeyName, _: Option<Unit>, _: SharedString) {}
        fn describe_histogram(&self, _: KeyName, _: Option<Unit>, _: SharedString) {}
        fn register_counter(&self, _: &Key, _: &Metadata) -> Counter { unimplemented!() }
        fn register_histogram(&self, _: &Key, _: &Metadata) -> Histogram { unimplemented!() }
    }

    let recorder = MetricRecorder::default();
    metrics::set_global_recorder(recorder.clone()).unwrap();

    let pool = pool();

    {
        let mut conn = pool.get().await.map_err(CacheError::GetConnection)?;
        let _: () = Cmd::new()
            .arg("FLUSHDB")
            .query_async(conn.deref_mut())
            .await?;
    }

    let cache = RedisCache::<Config>::new_with_pool(pool).await?;

    let create_channel = Event::ChannelCreate(Box::new(ChannelCreate(text_channel())));
    cache.update(&create_channel).await?;

    tokio::time::sleep(Config::METRICS_INTERVAL_DURATION + Duration::from_secs(1)).await;

    assert_eq!(recorder.render(), "channel_count: 1\nsticker_count: 0");

    let stickers = stickers();

    let guild_stickers_update = Event::GuildStickersUpdate(GuildStickersUpdate {
        guild_id: stickers[0].guild_id.unwrap(),
        stickers,
    });

    cache.update(&guild_stickers_update).await?;

    tokio::time::sleep(Config::METRICS_INTERVAL_DURATION + Duration::from_secs(1)).await;

    assert_eq!(recorder.render(), "channel_count: 1\nsticker_count: 2");

    Ok(())
}
