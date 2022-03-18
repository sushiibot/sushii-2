import { REST } from "@discordjs/rest";
import dotenv from "dotenv";
import { AMQPClient } from "@cloudamqp/amqp-client";
import log from "./logger";
import InteractionClient from "./interactions/client";
import UserInfoCommand from "./interactions/user/userinfo";
import { Config } from "./config";
import FishyCommand from "./interactions/user/fishy";
import AmqpGateway from "./gateway/amqp";
import initI18next from "./i18next";

async function main(): Promise<void> {
  dotenv.config();

  await initI18next();

  const config = new Config();
  const amqpClient = new AMQPClient(config.amqpUrl);
  const rabbitGatewayClient = new AmqpGateway(amqpClient, config);
  const rest = new REST({ version: "9" }).setToken(config.token);

  const interactionClient = new InteractionClient(rest, config);
  interactionClient.addCommand(new UserInfoCommand());
  interactionClient.addCommand(new FishyCommand());

  await interactionClient.register();

  log.info("connecting to rabbitmq for gateway events");
  rabbitGatewayClient.connect((msg) =>
    interactionClient.handleAMQPMessage(msg)
  );

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
