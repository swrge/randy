# Bot Worker

A Discord bot implementation using Google Cloud Functions to handle Discord interactions and forward requests to the bot-requester service.

## Project Structure

```
randy/bot-worker/
├── cmd/            # Entry point for local development
│   └── main.go     # Local server startup code
├── features/       # Command implementations and business logic
│   ├── slash_commands.go    # Routing for slash commands
│   └── weather_command.go   # Example weather command implementation 
├── http/           # HTTP client for communication with bot-requester
│   └── client.go   # HTTP wrapper for Discord API requests
├── function.go     # Cloud Function entry point and verification
├── helpers.go      # Utility functions for Discord interactions
├── go.mod          # Go module definition
└── go.sum          # Go dependencies checksum
```

## Architecture

This project consists of three main components:

1. **Cloud Function Handler** (`function.go`): Handles incoming Discord interactions, verifies signatures, and routes to the appropriate handler.

2. **Feature Handlers** (`features/`): Contains implementations for different commands and interaction types.

3. **HTTP Client** (`http/`): Provides a client for communicating with the bot-requester service.

## Adding New Commands

1. Create a new handler function in an existing file or create a new file in the `features/` directory.

2. Update the `HandleSlash` function in `features/slash_commands.go` to route to your new command:

```go
func HandleSlash(interaction disgo.Interaction, w http.ResponseWriter) {
    var command = interaction.ApplicationCommand()
    switch command.Name {
    case "ping":
        handlePing(interaction, w)
    case "my-new-command":
        handleMyNewCommand(interaction, w)
    default:
        // Default handler code...
    }
}
```

3. Implement your command handler function:

```go
func handleMyNewCommand(interaction disgo.Interaction, w http.ResponseWriter) {
    // 1. Extract command options if needed
    // 2. Send immediate response to Discord
    // 3. (Optional) Perform additional processing asynchronously
    // 4. (Optional) Forward requests to the bot-requester service
}
```

## Environment Variables

- `BOT_REQUESTER_URL`: URL of the bot-requester service (default: "http://localhost:8088")
- `PORT`: Port to run the local server on (default: "8080")
- `LOCAL_ONLY`: Set to "true" to only listen on localhost (for development)

## Development

### Prerequisites

- Go 1.19 or later
- Access to a Discord bot and its public key

### Running Locally

1. Set up environment variables:
```
export BOT_REQUESTER_URL=http://localhost:8088
export LOCAL_ONLY=true
export PORT=8080
```

2. Run the server:
```
cd cmd
go run main.go
```

3. Expose your local server (using ngrok or similar) to receive Discord interactions.

### Deploying to Google Cloud Functions

1. Build the function:
```
gcloud functions deploy DiscordInteraction \
  --runtime go119 \
  --trigger-http \
  --allow-unauthenticated
```

2. Set up Discord to send interactions to your deployed function URL.

## Security

- The `function.go` file contains signature verification to ensure requests are genuinely from Discord.
- Replace the placeholder public key in `function.go` with your actual Discord application public key.