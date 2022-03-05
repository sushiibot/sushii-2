import { REST } from "@discordjs/rest";
import { Client, Intents } from "discord.js";
import dotenv from "dotenv";
import log from "./logger";
import InteractionClient from "./interactions/client";
import UserInfoCommand from "./interactions/user/userinfo";
import {
  formModalHandler,
  formSlashCommand,
  formButtonHandler,
} from "./interactions/form/form";
import { Config } from "./config";

async function main() {
  dotenv.config();

  const config = new Config();
  const client = new Client({ intents: [Intents.FLAGS.GUILDS] });
  const rest = new REST({ version: "9" }).setToken(config.token);

  const interactionClient = new InteractionClient(rest, config);
  interactionClient.addCommand(new UserInfoCommand());
  interactionClient.addCommand(formSlashCommand);
  interactionClient.addModal(formModalHandler);
  interactionClient.addButton(formButtonHandler);

  await interactionClient.register();

  client.on("interactionCreate", (interaction) =>
    interactionClient.handleInteraction(interaction)
  );

  log.info("starting client");
  client.login(config.token);

  process.on("SIGINT", () => {
    log.info("cleaning up");

    client.destroy();
    log.info("bye");
    process.exit();
  });
}

main().catch((e) => {
  log.error("fatal error: %o", e);
});
