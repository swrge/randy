import { commands } from './commands/index.js';
import * as process from 'node:process';
import { createBot, Bot } from '@discordeno/bot';
import { createRequire } from 'node:module';
import { validateSnowflake } from './utils/snowflake.js';

const require = createRequire(import.meta.url);
const minimist = require('minimist');
// Parse command line arguments
const argv = minimist(process.argv.slice(2), {
  string: ['token', 'applicationId', 'restUrl'],
  boolean: ['global'],
  alias: {
    t: 'token',
    aid: 'applicationId',
    r: 'restUrl',
    G: 'global',
  },
  default: {
    global: false,
  },
});

// Extract non-option arguments into a single array
const guildIds = argv['_'].map(validateSnowflake);
// Function to validate Discord bot token format
function validateDiscordToken(token: string): boolean {
  // Discord tokens typically have 3 parts separated by periods
  // and are around 60-70 characters long
  if (!token || token === 'undefined' || token === 'null') {
    return false;
  }

  // Basic format check (doesn't validate the actual content)
  return /^[\w-]+\.[\w-]+\.[\w-]+$/.test(token);
}

// Prioritize command line arguments over environment variables
let TOKEN = String(argv.token || process.env.BOT_TOKEN || '');
const APPLICATION_ID = String(argv.applicationId) || String(process.env.APPLICATION_ID);

// Remove any whitespace or quotes that might have been accidentally included
TOKEN = TOKEN.trim().replace(/^['"]|['"]$/g, '');

// Add debug info about the token format
console.log(`Token length: ${TOKEN.length}`);
console.log(`Token format valid: ${validateDiscordToken(TOKEN)}`);

if (!TOKEN) {
  throw new Error(
    'The TOKEN is required. Provide it via environment variable or --token argument.'
  );
}

if (!validateDiscordToken(TOKEN)) {
  throw new Error(
    'The provided Discord token appears to be invalid. Discord tokens should be in the format "xxx.yyy.zzz".'
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

// Get REST URL
const restUrl = (argv.restUrl as string) || process.env.REST_URL;
console.log(`Using REST URL: ${restUrl || 'not provided, using default'}`);

let BOT: Bot;
try {
  const options: any = {
    token: TOKEN,
  };

  // Only add rest.proxy.baseUrl if a REST URL was provided
  if (restUrl) {
    options.rest = {
      proxy: {
        baseUrl: restUrl + '/api',
      },
    };
  }

  BOT = createBot(options);

  console.log('Bot created successfully');
} catch (error) {
  console.error('Failed to create bot:', error);
  process.exit(1);
}

export { BOT };

// Wrap operations in async function to handle top-level awaits
async function main() {
  if ((argv.global as boolean) === true) {
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
}

// Execute the main function
main().catch(error => {
  console.error('Error executing command registration:', error);
  if (error instanceof DOMException && error.name === 'InvalidCharacterError') {
    console.error(
      'This appears to be a token decoding error. Please check that your token is valid and properly formatted.'
    );
  }
  process.exit(1);
});
