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
import FishyCommand from "./interactions/user/fishy";
import AmqpGateway from "./gateway/amqp";
import { AMQPClient } from "@cloudamqp/amqp-client";

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
  const amqpClient = new AMQPClient(config.amqpUrl);
  const rabbitGatewayClient = new AmqpGateway(amqpClient, config);
  const rest = new REST({ version: "9" }).setToken(config.token);

  const interactionClient = new InteractionClient(rest, config);
  interactionClient.addCommand(new UserInfoCommand());
  interactionClient.addCommand(new FormSlashCommand());
  interactionClient.addCommand(new FishyCommand());
  interactionClient.addModal(formModalHandler);
  interactionClient.addButton(formButtonHandler);

  await interactionClient.register();

  log.info("connecting to rabbitmq for gateway events");
  rabbitGatewayClient.connect(interactionClient.handleAMQPMessage);

  process.on("SIGINT", () => {
    log.info("cleaning up");

    rabbitGatewayClient.stop();
    log.info("bye");
    process.exit();
  });
}

main().catch((e) => {
  log.error(e, "fatal error rip");
});
