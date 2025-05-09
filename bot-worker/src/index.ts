import {
  DiscordInteraction,
  InteractionResponse,
  InteractionResponseTypes,
  InteractionTypes,
} from '@discordeno/types';

import { verifyKey } from './utils/crypto';
//import { PING } from './commands/ping';

type CheckedInteraction = Promise<{
  interaction: DiscordInteraction | null;
  isValid: boolean;
}>;

interface Env {
  BOT_TOKEN: string;
  PUBLIC_KEY: string;
  APPLICATION_ID: string;
}

function unknownInteraction(): Response | PromiseLike<Response> {
  return new Response(
    JSON.stringify({
      type: InteractionResponseTypes.ChannelMessageWithSource,
      data: {
        content: 'Unknown interaction type',
      },
    }),
    {
      headers: { 'Content-Type': 'application/json' },
      status: 400,
    }
  );
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
async function handleSlashCommand(interaction: DiscordInteraction): Promise<Response> {
  const { data } = interaction;

  // Check which command was invoked
  switch (data?.name) {
    //case PING.name:
    //  return createResponse(handlePingCommand(interaction));

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
async function handleInteraction(interaction: DiscordInteraction): Promise<Response> {
  switch (interaction.type) {
    // Discord sends a ping to validate the endpoint
    case InteractionTypes.Ping:
      return createResponse({
        type: InteractionResponseTypes.Pong,
      });

    // Handle slash commands
    case InteractionTypes.ApplicationCommand:
      return handleSlashCommand(interaction);

    // Handle unknown interaction types
    default:
      return unknownInteraction();
  }
}

// Main worker handler
export default {
  async fetch(request: Request, env: Env, _ctx: ExecutionContext): Promise<Response> {
    // Only accept POST requests
    if (request.method !== 'POST') {
      return new Response('Method not allowed', { status: 405 });
    }

    // Verify the request is from Discord
    const isValidRequest = await verifyDiscordRequest(request, env);
    if (!isValidRequest) {
      return new Response('Unauthorized', { status: 401 });
    }

    // Clone the request to reuse the body
    const clonedRequest = request.clone();
    const body = (await clonedRequest.json()) as DiscordInteraction;

    // Handle the Discord interaction
    return handleInteraction(body);
  },
};
