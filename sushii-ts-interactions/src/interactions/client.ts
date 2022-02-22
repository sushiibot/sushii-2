import Collection from "@discordjs/collection";
import { REST } from "@discordjs/rest";
import {
  Routes,
  RESTPostAPIApplicationCommandsJSONBody,
} from "discord-api-types/v9";
import {
  CommandInteraction,
  Interaction,
  ModalSubmitInteraction,
} from "discord.js";
import { ConfigI } from "../config";
import Context from "../context";
import log from "../logger";
import { SlashCommand } from "./command";
import ModalHandler from "./modalHandler";

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
  private commands: Collection<string, SlashCommand>;

  /**
   * Modal handlers. This is only for *pure* handlers, if modals require some
   * side effects or logic, they should be handled in the command handler with
   * await modals.
   */
  private modals: Collection<string, ModalHandler>;

  constructor(rest: REST, config: ConfigI) {
    this.rest = rest;
    this.config = config;
    this.context = new Context();
    this.commands = new Collection();
    this.modals = new Collection();
  }

  /**
   * Add a new command to register and handle
   *
   * @param command SlashCommand to add
   */
  public addCommand(command: SlashCommand): void {
    this.commands.set(command.command.name, command);
  }

  /**
   * Add a pure modal handler
   *
   * @param modalHandler ModalHandler to add
   */
  public addModal(modalHandler: ModalHandler): void {
    this.modals.set(modalHandler.id, modalHandler);
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
    const modalHandler = this.modals.get(interaction.customId);

    if (!modalHandler) {
      log.error(
        "received unknown modal submit interaction: %s",
        interaction.customId
      );

      return;
    }

    log.info("received %s modal submit", interaction.customId);

    try {
      await modalHandler.handleSubmit(this.context, interaction);
    } catch (e) {
      log.error("error handling modal %s: %o", interaction.id, e);
    }
  }
}
