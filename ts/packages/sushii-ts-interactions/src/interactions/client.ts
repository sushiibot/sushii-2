import Collection from "@discordjs/collection";
import { REST } from "@discordjs/rest";
import {
  Routes,
  RESTPostAPIApplicationCommandsJSONBody,
} from "discord-api-types/v9";
import {
  ButtonInteraction,
  CommandInteraction,
  Interaction,
  ModalSubmitInteraction,
} from "discord.js";
import { ConfigI } from "../config";
import Context from "../context";
import log from "../logger";
import {
  ButtonHandler,
  SlashCommandHandler,
  InteractionHandler,
  ModalHandler,
} from "./handlers";

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
    this.context = new Context(config.dataApiURL);
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
   * Handle any interaction, eg slash commands
   *
   * @param interaction interaction from gateway
   * @returns
   */
  public async handleInteraction(interaction: Interaction): Promise<void> {
    if (interaction.isCommand()) {
      this.handleInteractionCommand(interaction);
    }

    if (interaction.isModalSubmit()) {
      this.handleModalSubmit(interaction);
    }

    if (interaction.isButton()) {
      this.handleButtonSubmit(interaction);
    }
  }

  /**
   * Handle a slash command
   *
   * @param interaction slash command interaction
   * @returns
   */
  private async handleInteractionCommand(
    interaction: CommandInteraction
  ): Promise<void> {
    const command = this.commands.get(interaction.commandName);

    if (!command) {
      log.error(`received unknown command: ${interaction.commandName}`);
      return;
    }

    log.info("received %s command", interaction.commandName);

    try {
      // Pre-check
      if (command.check) {
        const checkRes = await command.check(this.context, interaction);

        if (!checkRes.pass) {
          await interaction.reply(checkRes.message);

          log.info(
            "command %s failed check: %s",
            interaction.commandName,
            checkRes.message
          );
          return;
        }
      }

      await command.handler(this.context, interaction);
    } catch (e) {
      log.error("error running command %s: %o", interaction.commandName, e);
      await interaction.reply("Uh oh something broke");
    }
  }

  /**
   * Handle a pure modal submit interaction
   *
   * @param interaction modal submit interaction
   */
  private async handleModalSubmit(
    interaction: ModalSubmitInteraction
  ): Promise<void> {
    const modalHandler = this.modalHandlers.get(interaction.customId);

    if (!modalHandler) {
      log.error(
        "received unknown modal submit interaction: %s",
        interaction.customId
      );

      return;
    }

    log.info("received %s modal submit", interaction.customId);

    try {
      await modalHandler.handleModalSubmit(this.context, interaction);
    } catch (e) {
      log.error("error handling modal %s: %s", interaction.id, e);
    }
  }

  /**
   * Handle a pure button interaction
   *
   * @param interaction button interaction
   */
  private async handleButtonSubmit(
    interaction: ButtonInteraction
  ): Promise<void> {
    const buttonHandler = this.buttonHandlers.get(interaction.customId);

    if (!buttonHandler) {
      log.error(
        "received unknown button interaction: %s",
        interaction.customId
      );

      return;
    }

    log.info("received %s button", interaction.customId);

    try {
      await buttonHandler.handleButton(this.context, interaction);
    } catch (e) {
      log.error("error handling button %s: %o", interaction.id, e);
    }
  }
}
