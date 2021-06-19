use serenity::framework::standard::{macros::command, Args, CommandResult};
use serenity::model::prelude::*;
use serenity::prelude::*;
use std::collections::HashMap;
use std::fmt::Write;

use sushii_model::keys::DbPool;
use sushii_model::model::sql::GuildBan;
use sushii_model::model::sql::GuildConfig;

struct BanData {
    ban: GuildBan,
    guild_name: String,
    member_count: u64,
    features: Vec<String>,
    highest_feature: (u64, Option<&'static str>),
}

const FEATURE_ORDER: &[Option<&'static str>] = &[
    Some("PARTNERED"),
    Some("VERIFIED"),
    Some("DISCOVERABLE"),
    None,
];

fn get_server_feature_rank(features: &[String]) -> (u64, Option<&'static str>) {
    if features.contains(&"PARTNERED".to_string()) {
        (4, Some("PARTNERED"))
    } else if features.contains(&"VERIFIED".to_string()) {
        (3, Some("VERIFIED"))
    } else if features.contains(&"DISCOVERABLE".to_string()) {
        (2, Some("DISCOVERABLE"))
    } else {
        (1, None)
    }
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

    let target_id = match args.single::<u64>() {
        Ok(n) => n,
        Err(_) => {
            msg.channel_id
                .say(&ctx.http, "Error: Invalid user ID")
                .await?;

            return Ok(());
        }
    };

    let user = match UserId(target_id).to_user(ctx).await {
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

    let pool = ctx.data.read().await.get::<DbPool>().cloned().unwrap();

    let bans = GuildBan::lookup_user_id(&pool, UserId(target_id)).await?;

    if bans.is_empty() {
        msg.channel_id
            .send_message(ctx, |m| {
                m.embed(|e| {
                    e.title("No bans were found for user.");

                    e
                });

                m
            })
            .await?;

        return Ok(());
    }

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

        let highest_feature = get_server_feature_rank(&features);

        ban_data.push(BanData {
            ban,
            guild_name,
            member_count,
            features,
            highest_feature,
        });
    }

    let mut guild_groups: HashMap<Option<&'static str>, Vec<BanData>> = HashMap::new();

    for ban in ban_data {
        let entry = guild_groups
            .entry(ban.highest_feature.1)
            .or_insert_with(Vec::new);
        entry.push(ban);
    }

    for group in guild_groups.values_mut() {
        // Sort by member count
        group.sort_by(|a, b| a.member_count.cmp(&b.member_count));
    }

    let guild_config = GuildConfig::from_id(ctx, &guild_id).await?;
    let mut s = String::new();
    let mut total_anon_servers: u64 = 0;

    // If in override server
    let show_guild_names_global = guild_id.0 == 167058919611564043;

    for feature in FEATURE_ORDER.iter() {
        let bans = match guild_groups.get(feature) {
            Some(b) => b,
            None => continue,
        };

        if let Some(feature) = feature {
            writeln!(s, "**{} servers**", feature.to_lowercase())?;
        } else {
            writeln!(s, "**other servers**")?;
        }

        let mut anon_guilds: u64 = 0;

        for ban_data in bans {
            // Check if guild allows viewing
            // Global or curr guild AND other guild opted in
            let show_guild_name = show_guild_names_global
                || (guild_config
                    .as_ref()
                    .map(|c| c.data.lookup_details_opt_in)
                    .unwrap_or(false)
                    && GuildConfig::from_id(ctx, &GuildId(ban_data.ban.guild_id as u64))
                        .await?
                        .map(|c| c.data.lookup_details_opt_in)
                        .unwrap_or(false));

            if show_guild_name {
                write!(
                    s,
                    "> **{}** ({})",
                    ban_data.guild_name, ban_data.ban.guild_id
                )?;

                if let Some(ts) = ban_data.ban.action_time {
                    write!(s, " (<t:{}:R>)", ts.timestamp())?;
                }

                if let Some(reason) = &ban_data.ban.reason {
                    write!(s, ": {}", reason)?;
                }

                writeln!(s)?;
            } else {
                anon_guilds += 1;
                total_anon_servers += 1;
            }
        }

        if anon_guilds > 0 {
            writeln!(s, "{} anonymous server(s)", anon_guilds)?;
        }
    }

    msg.channel_id
        .send_message(ctx, |m| {
            m.embed(|e| {
                e.title("User Bans");
                e.author(|a| {
                    a.name(user.tag());
                    a.icon_url(user.face());

                    a
                });
                e.description(s);

                if !guild_config.as_ref().map(|c| c.data.lookup_details_opt_in).unwrap_or(false) && !show_guild_names_global {
                    e.footer(|f| {
                        f.text("Opted out of server name and reason sharing. \
                                To show other server name and reasons from other servers, use lookup optin");

                        f
                    });
                } else if total_anon_servers > 0 {
                    e.footer(|f| {
                        f.text("Anonymous servers have not opted into sharing server name and ban reasons.");

                        f
                    });
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
        msg.channel_id.say(ctx, "This server is already opted in! \
                                 Server name and ban reasons will be shared. \
                                 You can opt out with `lookup optout`.").await?;

        return Ok(());
    }

    guild_config.data.lookup_details_opt_in = true;
    guild_config.save(ctx).await?;

    msg.channel_id.say(ctx, "Opted in! Server name and ban reasons will be shared. \
                             You can opt out with `lookup optout`.").await?;

    Ok(())
}

#[command]
#[only_in("guild")]
#[required_permissions("BAN_MEMBERS")]
async fn optout(ctx: &Context, msg: &Message) -> CommandResult {
    let mut guild_config = GuildConfig::from_msg_or_respond(ctx, &msg).await?;

    if !guild_config.data.lookup_details_opt_in {
        msg.channel_id.say(ctx, "This server is already opted out! \
                                 Server name and ban reasons will not be shared. \
                                 You can opt in with `lookup optin`.").await?;

        return Ok(());
    }

    guild_config.data.lookup_details_opt_in = false;
    guild_config.save(ctx).await?;

    msg.channel_id.say(ctx, "Opted out! Server name and ban reasons will no longer be shared. \
                             You can opt in with `lookup optin`.").await?;

    Ok(())
}

