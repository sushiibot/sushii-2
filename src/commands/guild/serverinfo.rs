use num_format::{Locale, ToFormattedString};
use serenity::framework::standard::{macros::command, CommandResult};
use serenity::model::prelude::*;
use serenity::prelude::*;
use std::fmt::Write;

#[command]
#[aliases("guildinfo")]
#[only_in("guild")]
async fn serverinfo(ctx: &Context, msg: &Message) -> CommandResult {
    let guild = match msg.guild(ctx).await {
        Some(id) => id,
        None => {
            msg.channel_id.say(&ctx.http, "No guild found").await?;

            return Ok(());
        }
    };

    let owner = guild.owner_id.to_user(ctx).await?;

    let mut guild_str = String::new();

    if let Some(ref desc) = guild.description {
        writeln!(guild_str, "**Description:** {}", desc)?;
    }

    writeln!(guild_str, "**Owner:** {} (ID {})", owner.tag(), owner.id.0)?;
    writeln!(
        guild_str,
        "**Created:** {}",
        guild.id.created_at().format("%Y-%m-%d %H:%M:%S")
    )?;
    writeln!(
        guild_str,
        "**Members:** {}",
        guild.member_count.to_formatted_string(&Locale::en)
    )?;
    writeln!(guild_str, "**Region:** {}", guild.region)?;
    writeln!(guild_str, "**Roles:** {}", guild.roles.len())?;
    writeln!(
        guild_str,
        "**Verification Level:** {:?}",
        guild.verification_level
    )?;
    writeln!(
        guild_str,
        "**Explicit Content Filter:** {:?}",
        guild.explicit_content_filter
    )?;

    if !guild.features.is_empty() {
        write!(guild_str, "**Features:** ")?;

        for (i, feature) in guild.features.iter().enumerate() {
            write!(guild_str, "`{}`", feature)?;

            if i != guild.features.len() - 1 {
                write!(guild_str, ", ")?;
            }
        }

        writeln!(guild_str)?;
    }

    let (text_channels, voice_channels) = guild.channels.values().fold((0, 0), |mut acc, chan| {
        if chan.kind == ChannelType::Text {
            acc.0 += 1;
        } else if chan.kind == ChannelType::Voice {
            acc.1 += 1;
        };

        acc
    });

    writeln!(
        guild_str,
        "**Channels:** {} text, {} voice",
        text_channels, voice_channels
    )?;

    let (emojis, animated_emojis) = guild.emojis.values().fold((0, 0), |mut acc, emoji| {
        if emoji.animated {
            acc.1 += 1;
        } else {
            acc.0 += 1;
        }

        acc
    });

    writeln!(
        guild_str,
        "**Emojis:** {} static, {} animated",
        emojis, animated_emojis
    )?;

    if guild.premium_subscription_count > 0 {
        writeln!(
            guild_str,
            "**Boosts:** {}",
            guild.premium_subscription_count
        )?;
        writeln!(guild_str, "**Boost Level:** {}", guild.premium_tier.num())?;
    }

    if let Some(ref vanity_url_code) = guild.vanity_url_code {
        writeln!(
            guild_str,
            "**Vanity Invite:** [discord.gg/{code}](https://discord.gg/{code})",
            code = vanity_url_code
        )?;
    }

    msg.channel_id
        .send_message(ctx, |m| {
            m.embed(|e| {
                e.author(|a| {
                    a.name(&guild.name);

                    if let Some(url) = guild.icon_url() {
                        a.url(url);
                    }

                    a
                });

                if let Some(url) = guild.icon_url() {
                    e.thumbnail(url);
                }

                e.description(guild_str);

                if let Some(banner_url) = guild.banner_url() {
                    e.image(banner_url);
                }

                e.footer(|f| {
                    f.text(&format!("Guild ID: {}", &guild.id.0));

                    f
                });

                e
            })
        })
        .await?;

    Ok(())
}
