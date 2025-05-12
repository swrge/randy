//import { CreateApplicationCommand } from '@discordeno/types';
import { Interaction } from '@discordeno/bot';
import { Command } from './index';

async function handlePing(i: Interaction, _args: null) {
  return i.respond('pong');
}

export const PING: Command = {
  name: 'ping',
  description: 'Checks availability and latency.',
  execute: handlePing,
};
