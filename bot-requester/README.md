# bot-requester

## What this thing does

This is essentially a reverse proxy, it sits between the randy microservices and Discord's REST API to handle all the rate limiting stuff so we don't get the bot banned.

## Why this exists
- Can centralize rate limits for each microservice
- Can handle concurrent requests without conflicts


# How to use this

Because it mocks the Discord API, using it is as easy as changing the base of all routes from discord.com to localhost:XXXX, like this:

# Previously
$ curl https://discord.com/api/users/@me
# With the proxy
$ curl http://localhost:3000/api/users/@me

The proxy actively only supports routes from API v10 for now, older versions haven't been tested yet.

`twilight_http` has modular ratelimiter and support for proxying, so you can use it like this:
```rs
use twilight_http::Client;
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error + Send + Sync>> {
    let client = Client::builder()
        .proxy("localhost:3000", true)
        .ratelimiter(None)
        .build();

    Ok(())
}
```
This will use the proxy, skip the ratelimiter (since the proxy does ratelimiting itself), and will request over HTTP.

## Built with
- Go
- switchupcb/disgo discord api wrapper (handles all the rate limit logic)
- gorilla/mux for routing
- Google Cloud Run

## Setup
TODO

## Example endpoint

Currently implemented:
- POST `/api/channels/{channel_id}/messages` - sends a message to a channel

To use it, send a POST with JSON body:
```json
{
  "content": "Your message here"
}
```

## To add more endpoints

1. Add a new routeConfigs item in `routes.go`:
```go
   var routeConfigs = []RouteConfig{
	{
		Method:      "GET",
		PathPattern: EndpointGetGlobalApplicationCommands,
		RouteID:     "GetGlobalApplicationCommands",
		URLBuilder: func(vars map[string]string) string {
			return fmt.Sprintf("applications/%s/commands", vars["application_id"])
		},
	},
	...
   }
```

