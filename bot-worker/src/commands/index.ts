import { CreateApplicationCommand, CreateSlashApplicationCommand } from '@discordeno/types';
import { DiscordInteraction } from '@discordeno/bot';
import { PING } from './ping.js';

export const commands = new Map<string, CreateApplicationCommand>(
  [PING].map(cmd => [cmd.name, cmd])
);

export interface Command extends CreateSlashApplicationCommand {
  /** Handler that will be executed when this command is triggered */
  execute(i: DiscordInteraction, args: Record<string, unknown>): Promise<Response>;
}

export default commands;
