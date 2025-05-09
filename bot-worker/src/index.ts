import {
  DiscordInteraction,
  InteractionResponse,
  InteractionResponseTypes,
  InteractionTypes,
} from '@discordeno/types';

import { valueToUint8Array, concatUint8Arrays } from './utils/crypto';
import { webcrypto } from 'node:crypto';

// Environment variables interface for type safety
interface Env {
  BOT_TOKEN: string;
  PUBLIC_KEY: string;
  APPLICATION_ID: string;
}

async function verifyKey(
  rawBody: Uint8Array | ArrayBuffer | Buffer | string,
  signature: string,
  timestamp: string,
  clientPublicKey: string | webcrypto.CryptoKey
): Promise<boolean> {
  try {
    const timestampData = valueToUint8Array(timestamp);
    const bodyData = valueToUint8Array(rawBody);
    const message = concatUint8Arrays(timestampData, bodyData);
    const publicKey =
      typeof clientPublicKey === 'string'
        ? await webcrypto.subtle.importKey(
            'raw',
            valueToUint8Array(clientPublicKey, 'hex'),
            {
              name: 'ed25519',
              namedCurve: 'ed25519',
            },
            false,
            ['verify']
          )
        : clientPublicKey;
    const isValid = await webcrypto.subtle.verify(
      {
        name: 'ed25519',
      },
      publicKey,
      valueToUint8Array(signature, 'hex'),
      message
    );
    return isValid;
  } catch (ex) {
    return false;
  }
}

// Verify that the request is coming from Discord
async function verifyDiscordRequest(request: Request, env: Env): Promise<boolean> {
  const signature = request.headers.get('x-signature-ed25519');
  const timestamp = request.headers.get('x-signature-timestamp');
  const body = await request.text();

  const isValidRequest =
    signature && timestamp && (await verifyKey(body, signature, timestamp, env.PUBLIC_KEY));

  if (!isValidRequest) {
    return { isValid: false };
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
    case pingCommand.name:
      return createResponse(handlePingCommand(interaction));

    // Add more commands as needed
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
    case InteractionTypes.Ping:
      // Discord sends a ping to validate the endpoint
      return createResponse({
        type: InteractionResponseTypes.Pong,
      });

    case InteractionTypes.ApplicationCommand:
      // Handle slash commands
      return handleSlashCommand(interaction);

    default:
      // Handle unknown interaction types
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
