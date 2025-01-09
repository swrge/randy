# randy-ratelimiting

Ratelimiting functionality for HTTP requests for randy.
You have no reason to use this, please use `twilight-ratelimiting` instead.

Discord ratelimits requests to the HTTP API both globally and per-route.
For more information on the specifics, please take a look at
[Discord's documentation].


This crate provides a common [`Ratelimiter`] trait that all ratelimiter
implementations need to implement.

It also ships a default implementation, [`InMemoryRatelimiter`], that manages
the bucket states in memory.

In the future, randy will use the [`disgo`] ratelimiting implementation instead.

[Discord's documentation]: https://discord.com/developers/docs/topics/rate-limits
