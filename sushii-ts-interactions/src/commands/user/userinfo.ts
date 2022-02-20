import { SlashCommandBuilder } from "@discordjs/builders";
import {
    APIApplicationCommand,
    APIInteraction,
    GatewayInteractionCreateDispatch,
    InteractionType,
} from "discord-api-types/v9";
import { CommandInteraction, Interaction } from "discord.js";
import { log } from "../../logger";
import { SlashCommand } from "../command";

const cmd: SlashCommand = {
    command: new SlashCommandBuilder()
        .setName("userinfo")
        .setDescription("Get information about a user")
        .addUserOption((o) =>
            o
                .setName("user")
                .setDescription(
                    "The user to get information about, yourself if not provided"
                )
        )
        .toJSON(),
    handler: async (ctx: any, interaction: CommandInteraction) => {
        log.info("received userinfo command");

        interaction.reply("hi");
    },
};

export default cmd;
