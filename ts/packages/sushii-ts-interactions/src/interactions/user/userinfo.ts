import { SlashCommandBuilder, Embed } from "@discordjs/builders";
import { CacheType, CommandInteraction } from "discord.js";
import Context from "../../context";
import { SlashCommandHandler } from "../handlers";
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
    interaction: CommandInteraction<CacheType>
  ): Promise<void> {
    const target = interaction.options.getUser("user") || interaction.user;
    const member = await interaction.guild?.members.fetch(target.id);

    let authorName = target.username;
    if (member?.nickname) {
      authorName = `${target.username} ~ ${member.nickname}`;
    }

    // Force fetch to get banner
    await target.fetch(true);

    const embed = await getUserinfoEmbed(ctx, interaction, target, member);

    await interaction.reply({
      embeds: [embed],
    });
  }
}
