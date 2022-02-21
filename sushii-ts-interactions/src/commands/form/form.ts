import { SlashCommandBuilder, Embed } from "@discordjs/builders";
import {
  CommandInteraction,
  MessageActionRow,
  MessageButton,
  Modal,
  TextInputComponent,
} from "discord.js";
import { Context } from "../../context";
import { SlashCommand } from "../command";

const cmd: SlashCommand = {
  command: new SlashCommandBuilder()
    .setName("modapply")
    .setDescription("Apply for moderator")
    .toJSON(),
  handler: async (ctx: Context, interaction: CommandInteraction) => {
    const ageTextInput = new TextInputComponent()
      .setStyle("SHORT")
      .setCustomId("ageTextInput")
      .setLabel("Age")
      .setMinLength(2)
      .setMaxLength(2)
      .setRequired(true);

    const timezoneTextInput = new TextInputComponent()
      .setStyle("SHORT")
      .setCustomId("timezoneTextInput")
      .setLabel("Timezone")
      .setRequired(true);

    const whyTextInput = new TextInputComponent()
      .setStyle("PARAGRAPH")
      .setCustomId("whyTextInput")
      .setLabel("Do you moderate other servers?")
      .setMinLength(2)
      .setMaxLength(500)
      .setRequired(true);

    const rows = [ageTextInput, timezoneTextInput, whyTextInput].map((c) =>
      new MessageActionRow<TextInputComponent>().addComponents(c)
    );

    const modal = new Modal()
      .setTitle("Moderator Application")
      .setCustomId(`modal-${interaction.id}`)
      .addComponents(...rows);

    await interaction.showModal(modal);

    try {
      const submit = await interaction.awaitModalSubmit({
        filter: (i) => i.customId === `modal-${interaction.id}`,
        time: 10000,
      });
      console.log(submit.type);
    } catch (e) {
      console.error("Timed out");
    }
  },
};

export default cmd;
