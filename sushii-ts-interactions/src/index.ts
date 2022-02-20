import { REST } from "@discordjs/rest";
import { Client, Intents } from "discord.js";
import { log } from "./logger";
import { CommandClient } from "./commands/client";
import UserinfoCommand from "./commands/user/userinfo";
import { Config } from "./config";
import dotenv from "dotenv";

async function main() {
    dotenv.config();

    const config = new Config();
    const client = new Client({ intents: [Intents.FLAGS.GUILDS] });
    const rest = new REST({ version: "9" }).setToken(config.token);

    const cmdClient = new CommandClient(rest, config);
    cmdClient.addCommand(UserinfoCommand);

    await cmdClient.register();

    client.on("interactionCreate", cmdClient.handleInteraction);

    log.info("starting client");
    client.login(config.token);
}

main().catch((e) => {
    log.error(e);
});
