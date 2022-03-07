import { DiscordSnowflake } from "@sapphire/snowflake";

export function getCreatedTimestamp(snowflake: string): number {
  return DiscordSnowflake.timestampFrom(snowflake);
}

export function getCreatedTimestampSeconds(snowflake: string): number {
  return Math.floor(DiscordSnowflake.timestampFrom(snowflake) / 1000);
}

export function getCreatedDate(snowflake: string): Date {
  return new Date(getCreatedTimestamp(snowflake));
}
