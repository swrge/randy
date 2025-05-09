# Discord Bot Microservice on Cloudflare Workers

A serverless Discord bot slash command handler built as a microservice using Cloudflare Workers, TypeScript, and Discordeno.

## Features

- Handles Discord slash commands in a serverless environment
- Built on Cloudflare Workers for high availability and edge computing
- Written in TypeScript for type safety
- Uses Discordeno API types for Discord interaction handling
- Minimal dependencies for fast cold starts
- Structured for easy command addition and maintenance

## Prerequisites

- [Node.js](https://nodejs.org/) (version 16+)
- [Wrangler CLI](https://developers.cloudflare.com/workers/wrangler/install-and-update/) (Cloudflare Workers CLI)
- A Discord bot with slash commands enabled
- A Cloudflare account

## Setup

1. Clone the repository:
   ```bash
   git clone <repository-url>
   cd bot-worker2
   ```

2. Install dependencies:
   ```bash
   npm install
   ```

3. Configure your Discord bot credentials:
   ```bash
   cp wrangler.toml.example wrangler.toml
   # Edit wrangler.toml with your Discord Application ID and Public Key
   ```

4. Add your bot token as a secret:
   ```bash
   wrangler secret put BOT_TOKEN
   # Enter your Discord bot token when prompted
   ```

## Development

1. Start the local development server:
   ```bash
   npm run dev
   ```

2. Use tools like [ngrok](https://ngrok.com/) to expose your local server to the internet:
   ```bash
   ngrok http 8787
   ```

3. Update your Discord application's interaction endpoint URL to your ngrok URL.

## Command Structure

Commands are stored in the `src/commands` directory. Each command is a module that exports:

- A handler function that processes the interaction
- Command registration metadata

To add a new command:

1. Create a new file in `src/commands/`
2. Follow the pattern in `ping.ts`
3. Import and add your command to the index.ts switch statement
4. Add your command to the commands array in `register-commands.ts`

## Registering Commands with Discord

To register your commands with Discord:

```bash
# Create a registration script
node -e "require('./dist/utils/register-commands.js').registerCommandsFromScript()"
```

For development, register to a specific guild for instant updates:

```bash
TEST_GUILD_ID=your_guild_id node -e "require('./dist/utils/register-commands.js').registerCommandsFromScript()"
```

## Deployment

Deploy to Cloudflare Workers:

```bash
npm run deploy
```

After deployment, update your Discord application's interaction endpoint URL to your worker's URL.

## Security Considerations

- This project includes a placeholder for Discord interaction verification
- In production, implement proper request signature verification using Discord's public key
- All sensitive information should be stored as Cloudflare Worker secrets
- Review Cloudflare's security best practices

## Architecture

```
bot-worker2/
├── src/
│   ├── commands/        # Command definitions and handlers
│   ├── utils/           # Utility functions
│   └── index.ts         # Main worker entry point
├── wrangler.toml        # Cloudflare Worker configuration
└── tsconfig.json        # TypeScript configuration
```

## License

MIT