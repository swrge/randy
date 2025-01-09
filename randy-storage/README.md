# randy-storage

[![codecov badge][]][codecov link] [![github badge][]][github link] [![license badge][]][license link] ![rust badge]

`randy-storage` is an in-process-memory cache for randy.
It's responsible for processing events and caching things like guilds, channels, users, and voice states.
Do not use unless you know what you are doing, use `twilight-cache-inmemory` instead.

## Statistics

Statistics can be an important debugging tool for determining how large a
cache is or determining whether a cache has an expected amount of resources
within it. An interface for retrieving statistics about the amount of a
resource within the cache as a whole or on a guild-level can be retrieved
via [`InMemoryCache::stats`].

## Features

By default no feature is enabled.

### `permission-calculator`

The `permission-calculator` feature flag will bring in support for the
`PermissionCalculator`; an API for calculating permissions through it is
exposed via `InMemoryCache::permissions`. Support for calculating the
permissions of a member on a root guild-level and in a guild channel is
included.

Refer to the `permission` module for more documentation.

## Examples

Update a cache with events that come in through the gateway:

```rust,no_run
use std::{env, error::Error};
use randy_storage::DefaultInMemoryCache;
use randy_gateway::{EventTypeFlags, Intents, Shard, ShardId, StreamExt as _};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Initialize the tracing subscriber.
    tracing_subscriber::fmt::init();

    let token = env::var("DISCORD_TOKEN")?;
    let mut shard = Shard::new(ShardId::ONE, token, Intents::GUILD_MESSAGES);

    // Create a cache, caching up to 10 messages per channel:
    let cache = DefaultInMemoryCache::builder().message_cache_size(10).build();

    while let Some(item) = shard.next_event(EventTypeFlags::all()).await {
        let Ok(event) = item else {
            tracing::warn!(source = ?item.unwrap_err(), "error receiving event");

            continue;
        };

        // Update the cache with the event.
        cache.update(&event);
    }

    Ok(())
}
```

## License

All first-party crates are licensed under [ISC][LICENSE.md]

[LICENSE.md]: https://github.com/twilight-rs/twilight/blob/main/LICENSE.md
[`twilight-rs`]: https://github.com/twilight-rs/twilight
[codecov badge]: https://img.shields.io/codecov/c/gh/twilight-rs/twilight?logo=codecov&style=for-the-badge&token=E9ERLJL0L2
[codecov link]: https://app.codecov.io/gh/twilight-rs/twilight/
[github badge]: https://img.shields.io/badge/github-twilight-6f42c1.svg?style=for-the-badge&logo=github
[github link]: https://github.com/twilight-rs/twilight
[license badge]: https://img.shields.io/badge/license-ISC-blue.svg?style=for-the-badge&logo=pastebin
[license link]: https://github.com/twilight-rs/twilight/blob/main/LICENSE.md
[rust badge]: https://img.shields.io/badge/rust-1.79+-93450a.svg?style=for-the-badge&logo=rust
