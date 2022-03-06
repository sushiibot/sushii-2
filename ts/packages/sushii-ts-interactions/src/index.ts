import { REST } from "@discordjs/rest";
import { Client, Intents } from "discord.js";
import dotenv from "dotenv";
import log from "./logger";
import InteractionClient from "./interactions/client";
import UserInfoCommand from "./interactions/user/userinfo";
import {
  formModalHandler,
  FormSlashCommand,
  formButtonHandler,
} from "./interactions/form/form";
import { Config } from "./config";
import i18next from "i18next";
import Backend from "i18next-fs-backend";

async function main() {
  dotenv.config();

  await i18next.use(Backend).init({
    fallbackLng: "en",
    ns: ["commands"],
    defaultNS: "commands",
    backend: {
      loadPath: "/locales/{{lng}}/{{ns}}.json",
    },
  });

  const config = new Config();
  const client = new Client({ intents: [Intents.FLAGS.GUILDS] });
  const rest = new REST({ version: "9" }).setToken(config.token);

  const interactionClient = new InteractionClient(rest, config);
  interactionClient.addCommand(new UserInfoCommand());
  interactionClient.addCommand(new FormSlashCommand());
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
