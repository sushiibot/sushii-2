import { Embed } from "@discordjs/builders";
import { CacheType, CommandInteraction, GuildMember, User } from "discord.js";
import Context from "../../context";

export async function getUserinfoEmbed(
  ctx: Context,
  interaction: CommandInteraction<CacheType>,
  user: User,
  member: GuildMember | undefined
): Promise<Embed> {
  let authorName = user.username;
  if (member?.nickname) {
    authorName = `${user.username} ~ ${member.nickname}`;
  }

  // Force fetch to get banner
  await user.fetch(true);

  let embed = new Embed()
    .setAuthor({
      name: authorName,
      iconURL: user.displayAvatarURL({
        dynamic: true,
        size: 128,
      }),
      url: user.displayAvatarURL({
        dynamic: true,
        size: 4096,
      }),
    })
    .setThumbnail(user.displayAvatarURL())
    .setImage(
      user.bannerURL({
        dynamic: true,
        size: 512,
      })
    )
    .setFooter({
      text: `ID: ${user.id}`,
    });

  // Creation times
  embed = embed.addField({
    name: "Account Created",
    value: `<t:${user.createdTimestamp / 1000}:F> (<t:${
      user.createdTimestamp / 1000
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

  return embed;
}
