import { commands } from './commands';
import process from 'node:process';
import { createBot } from '@discordeno/bot';
import dotenv from 'dotenv';
import parseArgs from 'minimist';
import { validateSnowflake } from './utils/snowflake';
// Parse command line arguments
const argv = parseArgs(process.argv.slice(2), {
  string: ['token', 'applicationId', 'restUrl', 'authorization', 'envPath'],
  boolean: ['global'],
  alias: {
    t: 'token',
    aid: 'applicationId',
    r: 'restUrl',
    auth: 'authorization',
    P: 'envPath',
    G: 'global',
  },
  default: {
    envPath: '.dev.vars',
    global: false,
  },
});
// Load environment variables from file
dotenv.config({ path: argv.envPath });

// Extract non-option arguments into a single array
const guildIds = argv['_'].map(validateSnowflake);
// Prioritize command line arguments over environment variables
const TOKEN = String(argv.token) || String(process.env.TOKEN);
const APPLICATION_ID = String(argv.applicationId) || String(process.env.APPLICATION_ID);

if (!TOKEN) {
  throw new Error(
    'The TOKEN is required. Provide it via environment variable or --token argument.'
  );
}

if (!APPLICATION_ID) {
  throw new Error(
    'The APPLICATION_ID is required. Provide it via environment variable or --applicationId argument.'
  );
}

if (!guildIds.length && !argv.global) {
  throw new Error('At least one guild ID is required without --global...');
}

export const BOT = createBot({
  token: TOKEN,
  rest: {
    proxy: {
      baseUrl: argv.restUrl || process.env.REST_URL!,
      authorization: argv.authorization || process.env.AUTHORIZATION,
    },
  },
});

if (argv.global) {
  const data = await BOT.rest.upsertGlobalApplicationCommands([...commands.values()]);
  for (const command of data) {
    console.log(`Globally registered command ${command.name}`);
  }
} else {
  for (const guildId of guildIds) {
    const data = await BOT.rest.upsertGuildApplicationCommands(guildId, [...commands.values()]);
    for (const command of data) {
      console.log(`Registered command ${command.name} in guild ${guildId}`);
    }
  }
}
