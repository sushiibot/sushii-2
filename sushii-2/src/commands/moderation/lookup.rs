use serenity::framework::standard::{macros::command, Args, CommandResult};
use serenity::model::prelude::*;
use serenity::prelude::*;
use std::fmt::Write;

use sushii_model::keys::DbPool;
use sushii_model::model::sql::GuildBan;
use sushii_model::model::sql::GuildConfig;

use crate::utils::user::parse_id;

struct BanData {
    ban: GuildBan,
    guild_name: String,
    member_count: u64,
    anonymous: bool,
    features: Vec<String>,
}

fn get_server_emojis(features: &[String]) -> Vec<&'static str> {
    let mut emojis = Vec::new();

    if features.iter().any(|f| f.as_str() == "PARTNERED") {
        emojis.push("<:Partner:856045536687947796>")
    }

    if features.iter().any(|f| f.as_str() == "VERIFIED") {
        emojis.push("<:Verified:856045537236353024>")
    }

    if features.iter().any(|f| f.as_str() == "DISCOVERABLE") {
        emojis.push("🧭")
    }

    emojis
}

#[command]
#[only_in("guild")]
#[required_permissions("BAN_MEMBERS")]
async fn lookup(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let guild_id = match msg.guild_id {
        Some(id) => id,
        None => {
            msg.channel_id.say(&ctx.http, "Error: No guild").await?;

            return Ok(());
        }
    };

    let target_id = match args.single::<String>().ok().and_then(parse_id) {
        Some(id) => UserId(id),
        None => {
            msg.channel_id.say(&ctx, "Error: Invalid user ID").await?;

            return Ok(());
        }
    };

    let user = match target_id.to_user(ctx).await {
        Ok(u) => u,
        Err(_) => {
            msg.channel_id
                .say(
                    ctx,
                    "Error: Failed to fetch user, is this a correct user ID?",
                )
                .await?;

            return Ok(());
        }
    };

    let member = if let Some(guild) = msg.guild(ctx).await {
        if let Ok(member) = guild.member(ctx, target_id).await {
            Some(member)
        } else {
            None
        }
    } else {
        None
    };

    let pool = ctx.data.read().await.get::<DbPool>().cloned().unwrap();

    let bans = GuildBan::lookup_user_id(&pool, target_id).await?;

    if bans.is_empty() {
        msg.channel_id
            .send_message(ctx, |m| {
                m.embed(|e| {
                    e.author(|a| {
                        a.name(user.tag());
                        a.url(user.face());

                        a
                    });
                    e.thumbnail(user.face());
                    e.title("No bans were found for user.");

                    e.field(
                        "User Info",
                        format!(
                            "Account created at <t:{0}> (<t:{0}:R>)",
                            user.created_at().timestamp()
                        ),
                        false,
                    );

                    if let Some(member) = member {
                        if let Some(ref joined_at) = member.joined_at {
                            e.field(
                                "Member Info",
                                format!(
                                    "Joined server at <t:{0}> (<t:{0}:R>)",
                                    joined_at.timestamp(),
                                ),
                                false,
                            );
                        }
                    }

                    e
                });

                m
            })
            .await?;

        return Ok(());
    }

    let guild_config = GuildConfig::from_id(ctx, &guild_id).await?;

    // If in override server
    let show_guild_names_global =
        guild_id.0 == 167058919611564043
        || guild_id.0 == 184790855977009152
        || guild_id.0 == 187450744427773963;

    let mut ban_data = Vec::new();

    for ban in bans {
        let (guild_name, member_count, features) = match ctx
            .cache
            .guild_field(ban.guild_id as u64, |g| {
                (g.name.clone(), g.member_count, g.features.clone())
            })
            .await
        {
            Some(fields) => fields,
            None => ("Unknown".to_string(), 0, Vec::new()),
        };

        let show_guild_name = show_guild_names_global
            // Show if in current guild
            || Some(GuildId(ban.guild_id as u64)) == msg.guild_id
            || (guild_config
                .as_ref()
                .map(|c| c.data.lookup_details_opt_in)
                .unwrap_or(false)
                && GuildConfig::from_id(ctx, &GuildId(ban.guild_id as u64))
                    .await?
                    .map(|c| c.data.lookup_details_opt_in)
                    .unwrap_or(false));

        ban_data.push(BanData {
            ban,
            guild_name,
            member_count,
            anonymous: !show_guild_name,
            features,
        });
    }

    // Largest server first
    ban_data.sort_by(|a, b| b.member_count.cmp(&a.member_count));
    // Filter out small servers with less than 250 members
    let filtered_ban_data: Vec<&BanData> = ban_data
        .iter()
        .filter(|d| d.member_count > 250)
        .collect();

    let filtered_bans_count = ban_data.len() - filtered_ban_data.len();

    let mut s = String::new();
    let mut has_anon_servers = false;

    for ban_data in &filtered_ban_data {
        // Check if guild allows viewing
        // Global or curr guild AND other guild opted in

        let server_emojis = get_server_emojis(&ban_data.features);

        if !server_emojis.is_empty() {
            write!(s, "{}", server_emojis.join(" "))?;
        }

        if ban_data.anonymous {
            write!(s, "`anonymous`")?;
        } else {
            write!(
                s,
                "`{}` ({})",
                ban_data.guild_name.replace("`", "`\\``"),
                ban_data.ban.guild_id
            )?;
        }

        if let Some(ts) = ban_data.ban.action_time {
            write!(s, " (<t:{}:R>)", ts.timestamp())?;
        }

        if !ban_data.anonymous {
            if let Some(reason) = &ban_data.ban.reason {
                write!(s, ": {}", reason)?;
            }
        }

        if ban_data.anonymous {
            has_anon_servers = true;
        }

        writeln!(s)?;
    }

    if filtered_bans_count > 0 {
        writeln!(s)?;
        writeln!(s, "{} ban(s) in small servers not shown", filtered_bans_count)?;
    }

    msg.channel_id
        .send_message(ctx, |m| {
            m.embed(|e| {
                e.title(format!("{} bans found", ban_data.len()));
                e.author(|a| {
                    a.name(user.tag());
                    a.url(user.face());

                    a
                });
                e.thumbnail(user.face());
                e.description(s);

                if !guild_config.as_ref().map(|c| c.data.lookup_details_opt_in).unwrap_or(false) && !show_guild_names_global {
                    e.footer(|f| {
                        f.text("Opted out of server name and reason sharing. \
                                To show other server name and reasons from other servers, use lookup optin");

                        f
                    });
                } else if has_anon_servers {
                    e.footer(|f| {
                        f.text("Anonymous servers have not opted into sharing server name and ban reasons.");

                        f
                    });
                }

                e.field(
                    "User Info",
                    format!("Account created at <t:{0}> (<t:{0}:R>)", user.created_at().timestamp()),
                    false
                );

                if let Some(member) = member {
                    if let Some(ref joined_at) = member.joined_at {
                        e.field(
                            "Member Info",
                            format!(
                                "Joined server at <t:{0}> (<t:{0}:R>)",
                                joined_at.timestamp(),
                            ),
                            false
                        );
                    }
                }

                e
            });

            m
        })
        .await?;

    Ok(())
}

#[command]
#[only_in("guild")]
#[required_permissions("BAN_MEMBERS")]
async fn optin(ctx: &Context, msg: &Message) -> CommandResult {
    let mut guild_config = GuildConfig::from_msg_or_respond(ctx, &msg).await?;

    if guild_config.data.lookup_details_opt_in {
        msg.channel_id
            .say(
                ctx,
                "This server is already opted in! \
                 Server name and ban reasons will be shared. \
                 You can opt out with `lookup optout`.",
            )
            .await?;

        return Ok(());
    }

    guild_config.data.lookup_details_opt_in = true;
    guild_config.save(ctx).await?;

    msg.channel_id
        .say(
            ctx,
            "Opted in! Server name and ban reasons will be shared. \
             You can opt out with `lookup optout`.",
        )
        .await?;

    Ok(())
}

#[command]
#[only_in("guild")]
#[required_permissions("BAN_MEMBERS")]
async fn optout(ctx: &Context, msg: &Message) -> CommandResult {
    let mut guild_config = GuildConfig::from_msg_or_respond(ctx, &msg).await?;

    if !guild_config.data.lookup_details_opt_in {
        msg.channel_id
            .say(
                ctx,
                "This server is already opted out! \
                 Server name and ban reasons will not be shared. \
                 You can opt in with `lookup optin`.",
            )
            .await?;

        return Ok(());
    }

    guild_config.data.lookup_details_opt_in = false;
    guild_config.save(ctx).await?;

    msg.channel_id
        .say(
            ctx,
            "Opted out! Server name and ban reasons will no longer be shared. \
             You can opt in with `lookup optin`.",
        )
        .await?;

    Ok(())
}
