use serenity::framework::standard::{macros::command, Args, CommandResult};
use serenity::model::prelude::*;
use serenity::prelude::*;
use std::fmt::Write;

use crate::utils::user::parse_id;

#[command]
#[aliases("user")]
#[only_in("guild")]
async fn userinfo(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    let guild_id = match msg.guild_id {
        Some(id) => id,
        None => {
            msg.channel_id.say(&ctx.http, "No guild found").await?;

            return Ok(());
        }
    };

    let target_str = args.rest();

    let target_id = match parse_id(target_str) {
        Some(id) => UserId(id),
        None => {
            if !target_str.is_empty() {
                msg.channel_id
                    .say(ctx, "Error: Invalid user given.")
                    .await?;

                return Ok(());
            }

            // If empty use self
            msg.author.id
        }
    };

    let user = match target_id.to_user(&ctx.http).await {
        Ok(u) => u,
        Err(_) => {
            msg.reply(
                &ctx,
                "Error: Failed to fetch user, are you using a correct user ID?",
            )
            .await?;

            return Ok(());
        }
    };

    let member = guild_id.member(ctx, target_id).await;

    let mut user_str = String::new();

    if user.bot {
        writeln!(user_str, "(Bot)")?;
    } else {
        writeln!(user_str)?;
    }

    writeln!(user_str, "**ID:** {}", user.id.0)?;
    writeln!(
        user_str,
        "**Account created:** <t:{0}> (<t:{0}:R>)",
        user.created_at().timestamp()
    )?;

    let mut colour = None;

    if let Ok(member) = member {
        writeln!(user_str)?;
        writeln!(user_str, "**Member Info**")?;

        if let Some(ref nick) = member.nick {
            writeln!(user_str, "**Server Nickname:** {}", nick)?;
        }

        if let Some(ref joined_at) = member.joined_at {
            writeln!(
                user_str,
                "**Joined server:** <t:{0}> (<t:{0}:R>)",
                joined_at.timestamp(),
            )?;
        }

        if !member.roles.is_empty() {
            write!(user_str, "**Roles:** ")?;
        }

        colour = member.colour(ctx).await;

        let roles = match member.roles(ctx).await {
            Some(mut roles) => {
                roles.sort_by(|a, b| b.position.cmp(&a.position));
                roles.into_iter().map(|r| r.id).collect::<Vec<RoleId>>()
            }
            None => member.roles,
        };

        for role in &roles {
            write!(user_str, "{} ", role.mention())?;
        }

        if !roles.is_empty() {
            writeln!(user_str)?;
        }
    }

    msg.channel_id
        .send_message(ctx, |m| {
            m.embed(|e| {
                e.author(|a| {
                    a.name(&user.tag());
                    a.url(user.face());

                    a
                });

                if let Some(colour) = colour {
                    e.colour(colour);
                }

                e.thumbnail(user.face());
                e.description(user_str);

                if let Some(banner_url) = user.banner_url() {
                    e.image(banner_url);
                }

                if let Some(accent_color) = user.accent_color {
                    e.footer(|f| f.text(format!("Accent colour #{}", accent_color.hex())));
                }

                e
            })
        })
        .await?;

    Ok(())
}
