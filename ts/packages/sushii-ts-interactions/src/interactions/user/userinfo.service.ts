import { Embed } from "@discordjs/builders";
import {
  APIChatInputApplicationCommandInteraction,
  APIGuildMember,
  APIUser,
} from "discord-api-types/v9";
import Context from "../../context";
import { getCreatedTimestampSeconds } from "../../utils/snowflake";

export async function getUserinfoEmbed(
  ctx: Context,
  _interaction: APIChatInputApplicationCommandInteraction,
  user: APIUser,
  member: APIGuildMember | undefined
): Promise<Embed> {
  let authorName = user.username;
  if (member?.nick) {
    authorName = `${user.username} ~ ${member.nick}`;
  }

  let embed = new Embed()
    .setAuthor({
      name: authorName,
      iconURL: ctx.CDN.userFaceURL(user),
      url: ctx.CDN.userFaceURL(user),
    })
    .setThumbnail(ctx.CDN.userFaceURL(user))
    // Fine if they don't have banner
    .setImage(ctx.CDN.userBannerURL(user))
    .setFooter({
      text: `ID: ${user.id}`,
    });

  const createdTimestamp = getCreatedTimestampSeconds(user.id);

  // Creation times
  embed = embed.addField({
    name: "Account Created",
    value: `<t:${createdTimestamp}:F> (<t:${createdTimestamp}:R>)`,
  });

  if (member) {
    const joinedTimestamp = getCreatedTimestampSeconds(member.joined_at);
    embed = embed
      .addField({
        name: "Roles",
        value: member.roles.map((id) => `<@&${id}>`).join(" "),
      })
      // TODO: Display colour requires guild roles to be cached
      // .setColor(member.displayColor)
      .addField({
        name: "Joined Server",
        value: `<t:${joinedTimestamp}:F> (<t:${joinedTimestamp}:R>)`,
      });

    if (member.premium_since) {
      const premiumSinceTimestamp = getCreatedTimestampSeconds(
        member.premium_since
      );

      embed = embed.addField({
        name: "Boosting Since",
        value: `<t:${premiumSinceTimestamp}:F> (<t:${premiumSinceTimestamp}:R>)`,
      });
    }
  }

  return embed;
}
