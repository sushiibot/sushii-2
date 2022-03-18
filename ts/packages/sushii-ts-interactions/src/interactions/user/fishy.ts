import { SlashCommandBuilder, Embed } from "@discordjs/builders";
import { APIChatInputApplicationCommandInteraction } from "discord-api-types/v9";
import i18next from "i18next";
import Context from "../../context";
import { SlashCommandHandler } from "../handlers";
import CommandInteractionOptionResolver from "../resolver";
import { fishyForUser } from "./fishy.service";

export default class FishyCommand extends SlashCommandHandler {
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

  // eslint-disable-next-line class-methods-use-this
  async handler(
    ctx: Context,
    interaction: APIChatInputApplicationCommandInteraction
  ): Promise<void> {
    const options = new CommandInteractionOptionResolver(
      interaction.data.options,
      interaction.data.resolved
    );

    const target = options.getUser("user");
    if (!target) {
      await ctx.REST.interactionReplyMsg(interaction.id, interaction.token, {
        content: "You need to provide a user to fishy for!",
      });

      return;
    }

    const res = await fishyForUser(ctx, interaction, target);

    const embed = new Embed().setDescription(
      i18next.t("fishy.success", {
        ns: "commands",
        caughtType: res.caughtType,
        username: target.username,
        caughtAmount: res.caughtAmount,
        oldAmount: res.oldAmount,
        newAmount: res.newAmount,
      })
    );

    await ctx.REST.interactionReplyMsg(interaction.id, interaction.token, {
      embeds: [embed],
    });
  }
}
