use lazy_static::lazy_static;
use regex::Regex;
use serenity::builder::CreateEmbed;
use serenity::framework::standard::{macros::command, Args, CommandResult};
use serenity::model::prelude::*;
use serenity::prelude::*;

use crate::keys::CacheAndHttpContainer;
use crate::model::sql::{GuildConfig, GuildConfigDb, ModLogEntry, ModLogEntryDb};

#[command]
#[only_in("guild")]
async fn reason(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let guild_id = match msg.guild_id {
        Some(id) => id.0,
        None => {
            msg.channel_id.say(&ctx.http, "No guild found").await?;

            return Ok(());
        }
    };

    let range_str = match args.single::<String>() {
        Ok(s) => s,
        Err(_) => {
            msg.channel_id
                .say(&ctx.http, "Please give a case ID, ID range, or `latest`")
                .await?;

            return Ok(());
        }
    };

    let reason = args.rest();

    if reason.is_empty() {
        msg.channel_id
            .say(&ctx.http, "Please give a reason")
            .await?;

        return Ok(());
    }

    lazy_static! {
        // Limit to 19 chars max to prevent overflow, technically can have 20 digits but can still overflow
        // E.g.: u64 max: 18,446,744,073,709,551,615 -- Overflow with: 99,446,744,073,709,551,615
        static ref RE: Regex = Regex::new(r"(\d{1,19})\-(\d{1,19})").unwrap();
    }

    // Case ID range
    let (start, end) = if let Some(captures) = RE.captures(&range_str) {
        let start = captures.get(1).unwrap().as_str().parse::<u64>()?;
        let end = captures.get(2).unwrap().as_str().parse::<u64>()?;

        (Some(start), Some(end))

    // Single case ID
    } else if let Ok(num) = range_str.parse::<u64>() {
        // Just use the same start / end
        (Some(num), Some(num))

    // Latest
    } else if range_str == "latest" {
        (None, None)
    } else {
        msg.channel_id
            .say(
                &ctx.http,
                "Invalid case, please give a case ID (2), ID range (1-4), or `latest`",
            )
            .await?;

        return Ok(());
    };

    let entries = match (start, end) {
        (Some(start), Some(end)) => {
            ModLogEntry::get_range_entries(&ctx, guild_id, start, end).await?
        }
        (None, None) => ModLogEntry::get_latest(&ctx, guild_id)
            .await?
            .map_or_else(|| vec![], |entry| vec![entry]), // Just empty vec if None, or a Vec<Entry>
        _ => unreachable!(),
    };

    if entries.is_empty() {
        msg.channel_id
            .say(&ctx.http, "Error: No cases found with given ID")
            .await?;

        return Ok(());
    }

    let conf = GuildConfig::from_msg_or_respond(&ctx, msg).await?;
    let channel = match conf.log_mod {
        Some(c) => ChannelId(c as u64),
        None => {
            msg.channel_id
                .say(&ctx.http, "Error: There isn't a mod log channel set")
                .await?;

            return Ok(());
        }
    };

    let data = &ctx.data.read().await;
    let cache_http = data.get::<CacheAndHttpContainer>().unwrap();

    for mut entry in entries {
        let msg_id = match entry.msg_id {
            Some(id) => id,
            None => continue,
        };

        let mut message = match channel.message(&ctx.http, msg_id as u64).await {
            Ok(m) => m,
            Err(e) => {
                msg.channel_id
                    .say(
                        &ctx.http,
                        format!("Failed to fetch mod log message for case {}, maybe it doesn't exist or I can't read it",
                            entry.case_id)
                    ).await?;

                tracing::error!(?msg, "Failed to get mod log case message: {}", e);

                continue;
            }
        };

        let mut embed = match message.embeds.get(0) {
            Some(embed) => embed.clone(),
            None => {
                msg.channel_id
                    .say(
                        &ctx.http,
                        format!(
                            "Failed to fetch mod log message embed for case {}",
                            entry.case_id
                        ),
                    )
                    .await?;
                continue;
            }
        };

        // edit reason
        for mut field in &mut embed.fields {
            if field.name == "Reason" {
                field.value = reason.to_string();
            }
        }

        if let Err(e) = message
            .edit(&cache_http, |m| {
                m.embed(|e| {
                    *e = CreateEmbed::from(embed);
                    e.author(|a| {
                        a.name(msg.author.tag());
                        a.icon_url(msg.author.face());

                        a
                    });

                    e
                })
            })
            .await
        {
            msg.channel_id
                .say(
                    &ctx.http,
                    format!("Failed to edit mod log message for case {}", entry.case_id),
                )
                .await?;

            tracing::error!(?msg, "Failed to edit mod log message message: {}", e);
        }

        entry.reason.replace(reason.to_string());
        entry.executor_id.replace(msg.author.id.0 as i64);

        if let Err(e) = entry.save(&ctx).await {
            msg.channel_id
                .say(&ctx.http, format!("Failed to save case {}", entry.case_id))
                .await?;

            tracing::error!(?msg, "Failed to save mod log case: {}", e);
        }
    }

    msg.channel_id.say(&ctx, "Finished updating case reasons").await?;

    Ok(())
}
