function requiredEnv(envVar: string): string {
  const value = process.env[envVar];
  if (!value) {
    throw new Error(`Missing environment variable: ${envVar}`);
  }

  return value;
}

export interface ConfigI {
  token: string;
  applicationId: string;
  // If using guild commands for testing
  guildId: string | undefined;
  dataApiURL: string;
  amqpUrl: string;
  amqpQueueName: string;

  // Example: 'https://discord.com/api'
  proxyUrl: string;
}
export class Config implements ConfigI {
  public token: string;

  public applicationId: string;

  public guildId: string | undefined;

  public dataApiURL: string;

  public amqpUrl: string;

  public amqpQueueName: string;

  public proxyUrl: string;

  constructor() {
    this.token = requiredEnv("DISCORD_TOKEN");
    this.applicationId = requiredEnv("APPLICATION_ID");
    this.guildId = process.env.GUILD_ID;
    this.dataApiURL = requiredEnv("DATA_API_URL");
    this.amqpUrl = requiredEnv("AMQP_URL");
    this.amqpQueueName = requiredEnv("AMQP_QUEUE_NAME");
    this.proxyUrl = requiredEnv("TWILIGHT_PROXY_URL");
  }
}
