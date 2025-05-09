import { CreateApplicationCommand } from '@discordeno/types';
import { PING } from './ping';

export const commands = new Map<string, CreateApplicationCommand>(
  [PING].map(cmd => [cmd.name, cmd])
);

export default commands;
