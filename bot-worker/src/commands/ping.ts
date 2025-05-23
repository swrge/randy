//import { CreateApplicationCommand } from '@discordeno/types';
import { DiscordInteraction, InteractionResponse, InteractionResponseTypes } from '@discordeno/bot';
import { Command } from './index.js';
import { JsonResponse } from '../response.js';

export const PING: Command = {
  name: 'ping',
  description: 'Checks availability and latency.',
  execute: handlePing,
};

async function handlePing(
  _i: DiscordInteraction,
  _args: Record<string, unknown>
): Promise<Response> {
  const response: InteractionResponse = {
    type: InteractionResponseTypes.ChannelMessageWithSource,
    data: {
      content: 'Pong!',
    },
  };
  return new JsonResponse(response);
}
