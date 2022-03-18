import Collection from "@discordjs/collection";
import { REST } from "@discordjs/rest";
import {
  Routes,
  RESTPostAPIApplicationCommandsJSONBody,
  APIInteraction,
  APIChatInputApplicationCommandInteraction,
  APIModalSubmitInteraction,
  APIMessageComponentInteraction,
  InteractionType,
  ApplicationCommandType,
} from "discord-api-types/v9";
import { AMQPMessage } from "@cloudamqp/amqp-client";
import {
  isChatInputApplicationCommandInteraction,
  isMessageComponentButtonInteraction,
} from "discord-api-types/utils/v9";
import { ConfigI } from "../config";
import Context from "../context";
import log from "../logger";
import { ButtonHandler, SlashCommandHandler, ModalHandler } from "./handlers";
import { isGatewayInteractionCreateDispatch } from "../utils/interactionTypeGuards";

export default class InteractionClient {
  /**
   * Discord REST client
   */
  private rest: REST;

  /**
   * Bot configuration
   */
  private config: ConfigI;

  /**
   * Command context for shared stuff like database connections, API clients, etc.
   */
  private context: Context;

  /**
   * Command handlers
   */
  private commands: Collection<string, SlashCommandHandler>;

  /**
   * Modal handlers. This is only for *pure* handlers, if modals require some
   * side effects or logic, they should be handled in the command handler with
   * await modals.
   */
  private modalHandlers: Collection<string, ModalHandler>;

  /**
   * Button handlers
   */
  private buttonHandlers: Collection<string, ButtonHandler>;

  constructor(rest: REST, config: ConfigI) {
    this.rest = rest;
    this.config = config;
    this.context = new Context(config);
    this.commands = new Collection();
    this.modalHandlers = new Collection();
    this.buttonHandlers = new Collection();
  }

  /**
   * Add a new command to register and handle
   *
   * @param command SlashCommand to add
   */
  public addCommand(command: SlashCommandHandler): void {
    this.commands.set(command.command.name, command);
  }

  /**
   * Add a pure modal handler
   *
   * @param modalHandler ModalHandler to add
   */
  public addModal(modalHandler: ModalHandler): void {
    this.modalHandlers.set(modalHandler.modalId, modalHandler);
  }

  /**
   * Add a pure button handler
   *
   * @param buttonHandler ButtonHandler to add
   */
  public addButton(buttonHandler: ButtonHandler): void {
    this.buttonHandlers.set(buttonHandler.buttonId, buttonHandler);
  }

  /**
   *
   * @returns array of commands to register
   */
  private getCommandsArray(): RESTPostAPIApplicationCommandsJSONBody[] {
    return Array.from(this.commands.values()).map((c) => c.command);
  }

  /**
   * Register all slash commands via REST api
   *
   * @returns
   */
  public async register(): Promise<void> {
    log.info("registering %s guild commands", this.commands.size);

    // Actual global commands
    if (this.config.guildId === undefined) {
      await this.rest.put(
        Routes.applicationCommands(this.config.applicationId),
        { body: this.getCommandsArray() }
      );

      log.info("registered %s global commands", this.commands.size);
      return;
    }

    // Guild only commands for testing
    const res = await this.rest.put(
      Routes.applicationGuildCommands(
        this.config.applicationId,
        this.config.guildId
      ),
      { body: this.getCommandsArray() }
    );

    log.info("registered %s guild commands", this.commands.size, res);
  }

  /**
   * Handle a slash command
   *
   * @param interaction slash command interaction
   * @returns
   */
  private async handleInteractionCommand(
    interaction: APIChatInputApplicationCommandInteraction
  ): Promise<void> {
    const command = this.commands.get(interaction.data.name);

    if (!command) {
      log.error(`received unknown command: ${interaction.data.name}`);
      return;
    }

    log.info("received %s command", interaction.data.name);

    try {
      // Pre-check
      if (command.check) {
        const checkRes = await command.check(this.context, interaction);

        if (!checkRes.pass) {
          await this.context.REST.interactionReplyMsg(
            interaction.id,
            interaction.token,
            {
              content: checkRes.message,
            }
          );

          log.info(
            "command %s failed check: %s",
            interaction.data.name,
            checkRes.message
          );
          return;
        }
      }

      await command.handler(this.context, interaction);
    } catch (e) {
      log.error(e, "error running command %s", interaction.data.name);

      await this.context.REST.interactionReplyMsg(
        interaction.id,
        interaction.token,
        {
          content: "uh oh something broke",
        }
      );
    }
  }

  /**
   * Handle a pure modal submit interaction
   *
   * @param interaction modal submit interaction
   */
  private async handleModalSubmit(
    interaction: APIModalSubmitInteraction
  ): Promise<void> {
    const modalHandler = this.modalHandlers.get(interaction.data.custom_id);

    if (!modalHandler) {
      log.error(
        "received unknown modal submit interaction: %s",
        interaction.data.custom_id
      );

      return;
    }

    log.info("received %s modal submit", interaction.data.custom_id);

    try {
      await modalHandler.handleModalSubmit(this.context, interaction);
    } catch (e) {
      log.error(e, "error handling modal %s: %s", interaction.id);
    }
  }

  /**
   * Handle a pure button interaction
   *
   * @param interaction button interaction
   */
  private async handleButtonSubmit(
    interaction: APIMessageComponentInteraction
  ): Promise<void> {
    const buttonHandler = this.buttonHandlers.get(interaction.data.custom_id);

    if (!buttonHandler) {
      log.error(
        "received unknown button interaction: %s",
        interaction.data.custom_id
      );

      return;
    }

    log.info("received %s button", interaction.data.custom_id);

    try {
      await buttonHandler.handleButton(this.context, interaction);
    } catch (e) {
      log.error(e, "error handling button %s: %o", interaction.id);
    }
  }

  private async handleAPIInteraction(
    interaction: APIInteraction
  ): Promise<void> {
    if (interaction.type === InteractionType.ApplicationCommand) {
      if (isChatInputApplicationCommandInteraction(interaction)) {
        return this.handleInteractionCommand(interaction);
      }

      if (interaction.data.type === ApplicationCommandType.User) {
        // TODO: Handle user commands
      }

      if (interaction.data.type === ApplicationCommandType.Message) {
        // TODO: Handle message commands
      }
    }

    if (interaction.type === InteractionType.MessageComponent) {
      if (isMessageComponentButtonInteraction(interaction)) {
        return this.handleButtonSubmit(interaction);
      }
    }

    if (interaction.type === InteractionType.ModalSubmit) {
      return this.handleModalSubmit(interaction);
    }

    return undefined;
  }

  /**
   * Handles a raw gateway interaction from AMQP
   *
   * @param msg AMQP message
   * @returns
   */
  public async handleAMQPMessage(msg: AMQPMessage): Promise<void> {
    const msgString = msg.bodyToString();
    if (!msgString) {
      log.error("received empty AMQP message");
      return;
    }

    const interaction = JSON.parse(msgString);
    if (!isGatewayInteractionCreateDispatch(interaction)) {
      log.debug("received non-interaction AMQP message %s", interaction.t);
      return;
    }

    this.handleAPIInteraction(interaction.d);
  }
}
