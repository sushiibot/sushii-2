import { SlashCommandBuilder, Embed } from "@discordjs/builders";
import { CommandInteraction } from "discord.js";
import { Context } from "../../context";
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
  handler: async (ctx: Context, interaction: CommandInteraction) => {
    const target = interaction.options.getUser("user") || interaction.user;
    const member = await interaction.guild?.members.fetch(target.id);

    let authorName = target.username;
    if (member?.nickname) {
      authorName = `${target.username} ~ ${member.nickname}`;
    }

    // Force fetch to get banner
    await target.fetch(true);

    let embed = new Embed()
      .setAuthor({
        name: authorName,
        iconURL: target.displayAvatarURL({
          dynamic: true,
          size: 128,
        }),
        url: target.displayAvatarURL({
          dynamic: true,
          size: 4096,
        }),
      })
      .setThumbnail(target.displayAvatarURL())
      .setImage(
        target.bannerURL({
          dynamic: true,
          size: 512,
        })
      )
      .setFooter({
        text: `ID: ${target.id}`,
      });

    // Creation times
    embed = embed.addField({
      name: "Account Created",
      value: `<t:${target.createdTimestamp / 1000}:F> (<t:${
        target.createdTimestamp / 1000
      }:R>)`,
    });

    if (member) {
      embed = embed.addField({
        name: "Roles",
        value: member.roles.cache.map((r) => `<@&${r.id}>`).join(" "),
      });

      if (member.joinedTimestamp) {
        embed = embed.setColor(member.displayColor).addField({
          name: "Joined Server",
          value: `<t:${member.joinedTimestamp / 1000}:F> (<t:${
            member.joinedTimestamp / 1000
          }:R>)`,
        });
      }

      if (member.premiumSinceTimestamp) {
        embed = embed.addField({
          name: "Boosting Since",
          value: `<t:${member.premiumSinceTimestamp / 1000}:F> (<t:${
            member.premiumSinceTimestamp / 1000
          }:R>)`,
        });
      }
    }

    await interaction.reply({
      embeds: [embed],
    });
  },
};

export default cmd;
