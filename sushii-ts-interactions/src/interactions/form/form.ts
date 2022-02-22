import { SlashCommandBuilder, Embed } from "@discordjs/builders";
import {
  CommandInteraction,
  MessageActionRow,
  Modal,
  ModalSubmitFieldsResolver,
  ModalSubmitInteraction,
  TextInputComponent,
} from "discord.js";
import dayjs from "dayjs";
import Context from "../../context";
import { SlashCommand } from "../command";
import ModalHandler from "../modalHandler";

const MODAL_ID = "form:";

const AGE_CUSTOM_ID = "ageTextInput";
const TIMEZONE_CUSTOM_ID = "timezoneTextInput";
const OTHER_MOD_CUSTOM_ID = "modTextInput";

export const formSlashCommand: SlashCommand = {
  command: new SlashCommandBuilder()
    .setName("modapply")
    .setDescription("Apply for moderator")
    .toJSON(),
  handler: async (ctx: Context, interaction: CommandInteraction) => {
    const ageTextInput = new TextInputComponent()
      .setStyle("SHORT")
      .setCustomId(AGE_CUSTOM_ID)
      .setLabel("Age")
      .setMinLength(2)
      .setMaxLength(2)
      .setRequired(true);

    const timezoneTextInput = new TextInputComponent()
      .setStyle("SHORT")
      .setCustomId(TIMEZONE_CUSTOM_ID)
      .setLabel("Timezone")
      .setRequired(true);

    const whyTextInput = new TextInputComponent()
      .setStyle("PARAGRAPH")
      .setCustomId(OTHER_MOD_CUSTOM_ID)
      .setLabel("Do you moderate other servers?")
      .setMinLength(2)
      .setMaxLength(500)
      .setRequired(true);

    const rows = [ageTextInput, timezoneTextInput, whyTextInput].map((c) =>
      new MessageActionRow<TextInputComponent>().addComponents(c)
    );

    const modal = new Modal()
      .setTitle("Moderator Application")
      .setCustomId(MODAL_ID)
      .addComponents(...rows);

    await interaction.showModal(modal);
  },
};

export const formModalHandler: ModalHandler = {
  id: MODAL_ID,
  handleSubmit: async (ctx: Context, interaction: ModalSubmitInteraction) => {
    const resolver = new ModalSubmitFieldsResolver(interaction.components);

    const age = resolver.getTextInputValue(AGE_CUSTOM_ID);
    const tz = resolver.getTextInputValue(TIMEZONE_CUSTOM_ID);
    const otherMod = resolver.getTextInputValue(OTHER_MOD_CUSTOM_ID);

    let embed = new Embed()
      .setTitle("Moderator Application")
      .setAuthor({
        name: interaction.user.tag,
        iconURL: interaction.user.displayAvatarURL({ dynamic: true }),
      })
      .addField({
        name: "Age",
        value: age,
      })
      .addField({
        name: "Timezone",
        value: tz,
      })
      .addField({
        name: "Do you moderate other servers?",
        value: otherMod,
      });

    if (interaction.inRawGuild()) {
      const ts = dayjs(interaction.member.joined_at).unix();

      embed = embed.addField({
        name: "Member",
        value: `Joined <t:${ts}:R>`,
      });
    }

    await interaction.reply({
      embeds: [embed],
      ephemeral: true,
    });
  },
};
