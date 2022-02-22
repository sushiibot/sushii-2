import {
  SlashCommandBuilder,
  Embed,
  ButtonComponent,
} from "@discordjs/builders";
import {
  ButtonInteraction,
  CommandInteraction,
  MessageActionRow,
  Modal,
  ModalSubmitInteraction,
  TextInputComponent,
} from "discord.js";
import { ButtonStyle } from "discord-api-types/v9";
import Context from "../../context";
import { SlashCommand } from "../command";
import ModalHandler from "../modalHandler";
import ButtonHandler from "../buttonHandler";

const APPLY_BUTTON_ID = "button:apply";
const MODAL_ID = "form:";

const AGE_CUSTOM_ID = "ageTextInput";
const TIMEZONE_CUSTOM_ID = "timezoneTextInput";
const OTHER_MOD_CUSTOM_ID = "modTextInput";

export const formSlashCommand: SlashCommand = {
  command: new SlashCommandBuilder()
    .setName("modapply")
    .setDescription("Apply for moderator")
    .addStringOption((o) =>
      o.setName("message").setDescription("Message to send").setRequired(true)
    )
    .addChannelOption((o) =>
      o
        .setName("channel")
        .setDescription(
          "Channel to send this button to, defaults to the current channel"
        )
    )
    .toJSON(),
  handler: async (ctx: Context, interaction: CommandInteraction) => {
    const targetChannel =
      interaction.options.getChannel("channel") || interaction.channel;

    if (targetChannel?.type !== "GUILD_TEXT") {
      await interaction.reply("Invalid channel type, must be a text channel");
      return;
    }

    const button = new ButtonComponent()
      .setLabel("Apply")
      .setCustomId(APPLY_BUTTON_ID)
      .setStyle(ButtonStyle.Primary)
      .setEmoji({
        name: "📋",
      });

    const buttonRow = new MessageActionRow().addComponents(button);

    await targetChannel.send({
      content: interaction.options.getString("message"),
      components: [buttonRow],
    });

    await interaction.reply({
      content: "Form created!",
      ephemeral: true,
    });
  },
};

export const formButtonHandler: ButtonHandler = {
  id: APPLY_BUTTON_ID,
  handleButton: async (ctx: Context, interaction: ButtonInteraction) => {
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
    const age = interaction.fields.getTextInputValue(AGE_CUSTOM_ID);
    const tz = interaction.fields.getTextInputValue(TIMEZONE_CUSTOM_ID);
    const otherMod = interaction.fields.getTextInputValue(OTHER_MOD_CUSTOM_ID);

    let embed = new Embed()
      .setTitle("Moderator Application")
      .setAuthor({
        name: interaction.user.tag,
        iconURL: interaction.user.displayAvatarURL({ dynamic: true }),
      })
      .setDescription("Application received!")
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

    if (interaction.inCachedGuild() && interaction.member.joinedTimestamp) {
      embed = embed.addField({
        name: "Member",
        value: `Joined <t:${Math.floor(
          interaction.member.joinedTimestamp / 1000
        )}:R>`,
      });
    }

    await interaction.reply({
      embeds: [embed],
      ephemeral: true,
    });
  },
};
