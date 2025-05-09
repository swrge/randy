mod cache;
mod context;
//mod runner;
//mod session;
mod signals;

use bb8_redis::{
    bb8,
    redis::{cmd, AsyncCommands},
    RedisConnectionManager,
};
use cache::RedisConfig;
use context::Context;
use futures::executor::block_on;
use futures_util::stream::StreamExt;
use randy_gateway::{
    Config, ConfigBuilder, Event, EventTypeFlags, Intents, Message, Session, Shard, ShardId,
    StreamExt as _,
};
use randy_model::gateway::payload::incoming::{GuildCreate, Ready};
use randy_model::gateway::CloseFrame;
use randy_model::guild::{Guild, UnavailableGuild};
use randy_rest::Client;
use redlight::*;
use serde_json::to_string;
use std::sync::LazyLock;
use std::{collections::HashMap, pin::pin};
use std::{env, sync::atomic::AtomicBool};
use std::{
    panic::{set_hook, take_hook},
    pin::Pin,
};
use std::{sync::Arc, time::Duration};
use tokio::{runtime::Handle, time::timeout};

pub static SHUTDOWN: AtomicBool = AtomicBool::new(false);

static DEBUG: LazyLock<bool> =
    LazyLock::new(|| env::var("DEBUG").unwrap_or_else(|_| "false".to_string()) == "true");

#[rustfmt::skip]
static INTENTS: LazyLock<Intents> = LazyLock::new(|| {
    Intents::GUILD_MESSAGES
    | Intents::DIRECT_MESSAGES
    | Intents::MESSAGE_CONTENT
});

#[rustfmt::skip]
static FLAGS: LazyLock<EventTypeFlags> = LazyLock::new(|| {
    EventTypeFlags::READY
    | EventTypeFlags::GUILD_CREATE
    | EventTypeFlags::MEMBER_ADD
    | EventTypeFlags::MEMBER_UPDATE
    | EventTypeFlags::MEMBER_CHUNK
    | EventTypeFlags::MESSAGE_CREATE
    | EventTypeFlags::PRESENCE_UPDATE
    | EventTypeFlags::INTERACTION_CREATE
});

#[rustfmt::skip]
#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let var_name = println!("Starting bot...");
    let var_name = var_name;
    dotenvy::dotenv().expect("Failed to load environment variables");

    let token = env::var("DISCORD_TOKEN")
        .expect("Expected a token in the environment");
    println!("Token: {}", token);

    let proxy_url = env::var("PROXY_URL")
        .expect("PROXY_URL env var not set");
    println!("Proxy URL: {}", proxy_url);

    let client = Arc::new(
        Client::builder()
            .proxy(proxy_url, true)
            .ratelimiter(None)
            .build()
    );

    let redis_url = env::var("REDIS_URL")
        .expect("REDIS_URL env var error");
    println!("Redis URL: {}", redis_url);

    let manager = bb8_redis::RedisConnectionManager::new(redis_url)?;
    let pool = bb8_redis::bb8::Pool::builder().build(manager).await?;
    let cache = RedisCache::<RedisConfig>::new_with_pool(pool).await?;
    let cache = Arc::new(cache);
    let mut _conn = cache.pool().get().await?;
    println!("INFO: Redis cache configured");

    let mut builder = ConfigBuilder::from(Config::new(token, *INTENTS));
    let info = Context::thaw(&cache).await?;

    if let Some(session) = info.0 {
        builder = builder.session(session);
        println!("INFO: Session configured");
    }

    if let Some(url) = info.1 {
        builder = builder.resume_url(url);
        println!("INFO: Resume URL configured");
    }

    let mut shard_obj = Box::new(Shard::with_config(ShardId::ONE, builder.build()));
    let shard = Pin::new(&mut *shard_obj);
    let sender = shard.sender();
    println!("INFO: Shard configured");

    let ctx = Context::new_boxed(
        shard_obj,
        client.clone(),
        cache.clone()
    );

    println!("INFO: Context configured");

    println!("INFO: Running tasks...");
    let signal_handle = tokio::spawn(signals::on_signal(sender));
    let result = ctx.run().await;
    signal_handle.abort();
    let _ = signal_handle.await;

    if let Some(info) = result {
        if info.0.is_none() && info.1.is_none() {
            println!("INFO: No session or resume URL to freeze (both are None)");
        } else {
            if let Some(session) = info.0 {
                println!("Freezing session ID: {}", session.id());
                let mut sessions = HashMap::new();
                sessions.insert(ShardId::ONE.number(), session);
                cache.freeze(&sessions, None).await
                    .expect("Failed to freeze session");
            }

            if let Some(url) = info.1 {
                let mut conn = cache.connection().await
                    .expect("Failed to get connection");
                println!("Freezing resume URL: {}", &url);
                conn.set::<_, _, ()>("resume_url", &url).await
                    .expect("Failed to set resume URL");
            }
        }
    }

    Ok(())
}
