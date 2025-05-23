import { DiscordInteraction, InteractionResponseTypes, InteractionTypes } from '@discordeno/types';

import { verifyKey } from './utils/crypto.js';
import { PING } from './commands/ping.js';
import * as response from './response.js';

type CheckedInteraction = Promise<{
  interaction: DiscordInteraction | null;
  isValid: boolean;
}>;

export interface Env {
  BOT_TOKEN: string;
  REQUESTER_URL: string;
  PUBLIC_KEY: string;
  APPLICATION_ID: string;
}

// Verify that the request is coming from Discord
export async function verifyDiscordRequest(request: Request, env: Env): CheckedInteraction {
  const signature = request.headers.get('x-signature-ed25519');
  const timestamp = request.headers.get('x-signature-timestamp');
  const body = await request.text();

  const isValidRequest =
    signature && timestamp && (await verifyKey(body, signature, timestamp, env.PUBLIC_KEY));

  if (!isValidRequest) {
    return { interaction: null, isValid: false };
  }

  return { interaction: JSON.parse(body), isValid: true };
}

// Helper function to create a response from an interaction type and data
function createResponse(type: InteractionResponseTypes, data?: any): Response {
  return new Response(
    JSON.stringify({
      type,
      data,
    }),
    {
      headers: { 'Content-Type': 'application/json' },
    }
  );
}

// Handle Discord slash commands
async function handleSlashCommand(interaction: DiscordInteraction): Promise<Response> {
  switch (interaction.data?.name) {
    case PING.name:
      return await PING.execute(interaction, {});

    default:
      return createResponse(InteractionResponseTypes.ChannelMessageWithSource, {
        content: 'Unknown command',
      });
  }
}

export async function handleBotEvent(_request: Request, _env: Env): Promise<Response> {
  throw new Error('Function not implemented.');
}

export async function handleInteraction(request: Request, env: Env): Promise<Response> {
  const { interaction, isValid } = await verifyDiscordRequest(request, env);
  if (!isValid || !interaction) {
    return new Response('Unauthorized', { status: 401 });
  }

  console.log('Interaction received');

  switch (interaction.type) {
    case InteractionTypes.Ping:
      return createResponse(InteractionResponseTypes.Pong);

    case InteractionTypes.ApplicationCommand:
      return await handleSlashCommand(interaction);

    default:
      return await response.UnknownInteraction();
  }
}

// Main worker
export async function run(request: Request, env: Env, _ctx: ExecutionContext): Promise<Response> {
  const pathname = new URL(request.url).pathname;
  if (pathname.startsWith('/interactions')) {
    return await handleInteraction(request, env);
  } else if (pathname.startsWith('/bot-events')) {
    return await handleBotEvent(request, env);
  }
  if (request.method === 'GET') {
    return new Response(`👋 ${env.APPLICATION_ID}`);
  }
  return new Response('Not found', { status: 404 });
}

export default { fetch: run };
