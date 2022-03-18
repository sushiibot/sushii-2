import { SlashCommandBuilder } from "@discordjs/builders";
import {
  isDMInteraction,
  isGuildInteraction,
} from "discord-api-types/utils/v9";
import { APIChatInputApplicationCommandInteraction } from "discord-api-types/v9";
import Context from "../../context";
import logger from "../../logger";
import { SlashCommandHandler } from "../handlers";
import CommandInteractionOptionResolver from "../resolver";
import getUserinfoEmbed from "./userinfo.service";

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

  // eslint-disable-next-line class-methods-use-this
  async handler(
    ctx: Context,
    interaction: APIChatInputApplicationCommandInteraction
  ): Promise<void> {
    const options = new CommandInteractionOptionResolver(
      interaction.data.options,
      interaction.data.resolved
    );

    logger.debug("userinfo options: %o", options);

    let target = options.getUser("user");
    let member;

    logger.debug("userinfo option target user: %o", target);

    if (isGuildInteraction(interaction)) {
      if (!target) {
        target = interaction.member.user;
      }

      member = await ctx.REST.getMember(interaction.guild_id, target.id);

      logger.debug("userinfo option target member: %o", member);
    } else if (isDMInteraction(interaction)) {
      if (!target) {
        target = interaction.user;
      }
    }

    if (!target) {
      throw new Error("No target set, should be unreachable");
    }

    const embed = await getUserinfoEmbed(ctx, interaction, target, member);
    logger.debug("userinfo embed: %o", embed);

    await ctx.REST.interactionReplyMsg(interaction.id, interaction.token, {
      embeds: [embed],
    });
  }
}
