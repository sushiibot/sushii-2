export interface ConfigI {
    token: string;
    clientId: string;
    // If using guild commands for testing
    guildId: string | undefined;
}

export class Config implements ConfigI {
    public token: string;
    public clientId: string;
    public guildId: string | undefined;

    constructor() {
        this.token = requiredEnv("DISCORD_TOKEN");
        this.clientId = requiredEnv("CLIENT_ID");
        this.guildId = process.env.GUILD_ID;
    }
}

function requiredEnv(envVar: string): string {
    const value = process.env[envVar];
    if (!value) {
        throw new Error(`Missing environment variable: ${envVar}`);
    }

    return value;
}
