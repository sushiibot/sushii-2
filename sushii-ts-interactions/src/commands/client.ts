import Collection from "@discordjs/collection";
import { REST } from "@discordjs/rest";
import { Routes, APIApplicationCommand } from "discord-api-types/v9";
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

    public addCommand(command: SlashCommand): void {
        this.commands.set(command.command.name, command);
    }

    public async register(): Promise<undefined> {
        log.info(`registering {} guild commands`, this.commands.size);

        // Actual global commands
        if (this.config.guildId === undefined) {
            await this.rest.put(
                Routes.applicationCommands(this.config.clientId),
                { body: this.commands }
            );

            log.info(`registered {} global commands`, this.commands.size);
            return;
        }

        // Guild only commands for testing
        await this.rest.put(
            Routes.applicationGuildCommands(
                this.config.clientId,
                this.config.guildId
            ),
            { body: this.commands }
        );

        log.info(`registered {} guild commands`, this.commands.size);
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

        await command.handler(this.context, interaction);
    }
}
