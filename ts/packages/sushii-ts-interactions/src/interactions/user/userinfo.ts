import { SlashCommandBuilder } from "@discordjs/builders";
import {
  isDMInteraction,
  isGuildInteraction,
} from "discord-api-types/utils/v9";
import {
  APIChatInputApplicationCommandDMInteraction,
  APIChatInputApplicationCommandGuildInteraction,
  APIChatInputApplicationCommandInteraction,
} from "discord-api-types/v9";
import Context from "../../context";
import { SlashCommandHandler } from "../handlers";
import CommandInteractionOptionResolver from "../resolver";
import { getUserinfoEmbed } from "./userinfo.service";

export default class UserinfoHandler extends SlashCommandHandler {
  serverOnly = false;
  command = new SlashCommandBuilder()
    .setName("userinfo")
    .setDescription("Get information about a user")
    .addUserOption((o) =>
      o
        .setName("user")
        .setDescription(
          "The user to get information about, yourself if not provided"
        )
    )
    .toJSON();

  async handler(
    ctx: Context,
    interaction: APIChatInputApplicationCommandInteraction
  ): Promise<void> {
    const options = new CommandInteractionOptionResolver(
      interaction.data.options,
      interaction.data.resolved
    );

    let target = options.getUser("user");
    let member;

    if (isGuildInteraction(interaction)) {
      if (!target) {
        target = interaction.member.user;
      }

      member = await ctx.REST.getMember(interaction.guild_id, target.id);
    } else if (isDMInteraction(interaction)) {
      if (!target) {
        target = interaction.user;
      }
    }

    if (!target) {
      throw new Error("No target set, should be unreachable");
    }

    const embed = await getUserinfoEmbed(ctx, interaction, target, member);

    await ctx.REST.interactionReplyMsg(interaction.id, interaction.token, {
      embeds: [embed],
    });
  }
}
