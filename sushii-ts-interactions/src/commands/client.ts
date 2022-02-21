import Collection from "@discordjs/collection";
import { REST } from "@discordjs/rest";
import {
    Routes,
    APIApplicationCommand,
    RESTPostAPIApplicationCommandsJSONBody,
} from "discord-api-types/v9";
import { Interaction } from "discord.js";
import { ConfigI } from "../config";
import { Context } from "../context";
import { log } from "../logger";
import { SlashCommand } from "./command";

export class CommandClient {
    private rest: REST;
    private config: ConfigI;
    private context: Context;
    private commands: Collection<string, SlashCommand>;

    constructor(rest: REST, config: ConfigI) {
        this.rest = rest;
        this.config = config;
        this.context = new Context();
        this.commands = new Collection();
    }

    /**
     * Add a new command to register and handle
     *
     * @param command SlashCommand to add
     */
    public addCommand(command: SlashCommand): void {
        this.commands.set(command.command.name, command);
    }

    private getCommandsArray(): RESTPostAPIApplicationCommandsJSONBody[] {
        return Array.from(this.commands.values()).map((c) => c.command);
    }

    public async register(): Promise<undefined> {
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

    public async handleInteraction(interaction: Interaction): Promise<void> {
        if (!interaction.isCommand()) {
            return;
        }

        const command = this.commands.get(interaction.commandName);

        if (!command) {
            log.warn(`received unknown command: ${interaction.commandName}`);
            return;
        }

        log.info("received %s command", interaction.commandName);
        try {
            await command.handler(this.context, interaction);
        } catch (e) {
            log.error("error running command %s", interaction.commandName, e);
            interaction.reply("Uh oh something broke");
        }
    }
}
