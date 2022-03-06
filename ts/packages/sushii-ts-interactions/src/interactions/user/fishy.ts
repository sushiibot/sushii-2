import { SlashCommandBuilder, Embed } from "@discordjs/builders";
import { CacheType, CommandInteraction } from "discord.js";
import Context from "../../context";
import { SlashCommandHandler } from "../handlers";
import { fishyForUser } from "./fishy.service";
import i18next from "i18next";

export default class FishyHandler extends SlashCommandHandler {
  serverOnly = true;
  command = new SlashCommandBuilder()
    .setName("fishy")
    .setDescription("Catch some fish!")
    .addUserOption((o) =>
      o
        .setName("user")
        .setDescription("Who to fishy for or yourself if you have no friends")
        .setRequired(true)
    )
    .toJSON();

  async handler(
    ctx: Context,
    interaction: CommandInteraction<CacheType>
  ): Promise<void> {
    const target = interaction.options.getUser("user");
    if (!target) {
      return interaction.reply("You need to provide a user to fishy for!");
    }

    const res = await fishyForUser(ctx, interaction, target);

    const embed = new Embed().setDescription(
      i18next.t("fishy.success", {
        ns: "commands",
        caughtType: res.caughtType,
        username: target.username,
        caughtAmount: res.caughtAmount,
      })
    );

    await interaction.reply({
      embeds: [embed],
    });
  }
}
