import {
  DiscordInteraction,
  InteractionResponse,
  InteractionResponseTypes,
  InteractionTypes,
} from '@discordeno/types';

import { verifyKey } from './utils/crypto';
import { Bot, createBot, Interaction } from '@discordeno/bot';
import { PING } from './commands/ping';
import * as response from './response';

type CheckedInteraction = Promise<{
  interaction: DiscordInteraction | null;
  isValid: boolean;
}>;

interface Env {
  BOT_TOKEN: string;
  GATEWAY_URL: string;
  REQUESTER_URL: string;
  PUBLIC_KEY: string;
  APPLICATION_ID: string;
}

// Verify that the request is coming from Discord
async function verifyDiscordRequest(request: Request, env: Env): CheckedInteraction {
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

// Helper function to create a response from an interaction response
function createResponse(response: InteractionResponse): Response {
  return new Response(JSON.stringify(response), {
    headers: { 'Content-Type': 'application/json' },
  });
}

// Handle Discord slash commands
async function handleSlashCommand(i: Interaction): Promise<Response> {
  // Check which command was invoked
  switch (i.data?.name) {
    case PING.name:
      return await PING.execute(i, {});

    default:
      return createResponse({
        type: InteractionResponseTypes.ChannelMessageWithSource,
        data: {
          content: 'Unknown command',
        },
      });
  }
}

// Handle different types of interactions
async function handleInteraction(i: Interaction): Promise<Response> {
  switch (i.type) {
    // Discord sends a ping to validate the endpoint
    case InteractionTypes.Ping:
      return createResponse({
        type: InteractionResponseTypes.Pong,
      });

    // Handle slash commands
    case InteractionTypes.ApplicationCommand:
      return handleSlashCommand(i);

    // Handle unknown interaction types
    default:
      return response.UnknownInteraction();
  }
}

// Main worker
async function run(request: Request, env: Env, _ctx: ExecutionContext): Promise<Response> {
  // get wave to verify the worker is working.
  if (request.method === 'GET') {
    return new Response(`👋 ${env.APPLICATION_ID}`);
  }

  // Verify the request is from Discord
  const { interaction, isValid } = await verifyDiscordRequest(request, env);
  if (!isValid) {
    return new Response('Unauthorized', { status: 401 });
  }
  if (!interaction) {
    return new Response('Invalid interaction', { status: 400 });
  }

  // Handle the Discord interaction
  const bot: Bot = createBot({
    token: env.BOT_TOKEN,
    rest: {
      proxy: {
        baseUrl: env.REQUESTER_URL,
      },
    },
  });

  const i = bot.transformers.interaction(bot, { interaction, shardId: 0 }) as Interaction;
  return await handleInteraction(i);
}

export default { fetch: run };
