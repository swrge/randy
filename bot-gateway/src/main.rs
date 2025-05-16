mod cache;
mod context;
//mod runner;
//mod session;
mod signals;

use bb8_redis::redis::AsyncCommands;
use cache::RedisConfig;
use context::Context;
use randy_gateway::{Config, ConfigBuilder, EventTypeFlags, Intents, Shard, ShardId};
use randy_rest::Client;
use redlight::*;
use std::collections::HashMap;
use std::pin::Pin;
use std::sync::Arc;
use std::sync::LazyLock;
use std::{env, sync::atomic::AtomicBool};

#[allow(non_snake_case, dead_code)]
pub struct Env {
    BOT_TOKEN: LazyLock<String>,
    GATEWAY_URL: LazyLock<String>,
    REQUESTER_URL: LazyLock<String>,
    PUBLIC_KEY: LazyLock<String>,
    APPLICATION_ID: LazyLock<String>,
}

pub static ENV: Env = Env {
    BOT_TOKEN: LazyLock::new(|| {
        dotenvy::dotenv().ok();
        std::env::var("BOT_TOKEN").expect("missing environment variable")
    }),
    GATEWAY_URL: LazyLock::new(|| {
        dotenvy::dotenv().ok();
        std::env::var("GATEWAY_URL").expect("missing environment variable")
    }),
    REQUESTER_URL: LazyLock::new(|| {
        dotenvy::dotenv().ok();
        std::env::var("REQUESTER_URL").expect("missing environment variable")
    }),
    PUBLIC_KEY: LazyLock::new(|| {
        dotenvy::dotenv().ok();
        std::env::var("PUBLIC_KEY").expect("missing environment variable")
    }),
    APPLICATION_ID: LazyLock::new(|| {
        dotenvy::dotenv().ok();
        std::env::var("APPLICATION_ID").expect("missing environment variable")
    }),
};

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
    println!("Starting bot...");
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
