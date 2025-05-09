// MIT License
//
// Copyright (c) 2017-2021 Devin Spikowski
// Copyright (c) 2024-2025 swrge
//
// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in all
// copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
// SOFTWARE.

export const DISCORD_EPOCH = 1420070400000;

// Converts a snowflake ID string into a JS Date object using the provided epoch (in ms), or Discord's epoch if not provided
export function convertSnowflakeToDate(snowflake: string, epoch = DISCORD_EPOCH) {
  // Convert snowflake to BigInt to extract timestamp bits
  // https://discord.com/developers/docs/reference#snowflakes
  const milliseconds = BigInt(snowflake) >> 22n;
  return new Date(Number(milliseconds) + epoch);
}

// Validates a snowflake ID string and returns a JS Date object if valid
export function validateSnowflake(snowflake: string, epoch = DISCORD_EPOCH): string {
  if (!Number.isInteger(+snowflake)) {
    throw new Error("That doesn't look like a snowflake. Snowflakes contain only numbers.");
  }

  if (BigInt(snowflake) < 4194304n) {
    throw new Error("That doesn't look like a snowflake. Snowflakes are much larger numbers.");
  }

  const timestamp = convertSnowflakeToDate(snowflake, epoch);

  if (Number.isNaN(timestamp.getTime())) {
    throw new Error("That doesn't look like a snowflake. Snowflakes have fewer digits.");
  }
  return snowflake;
}
