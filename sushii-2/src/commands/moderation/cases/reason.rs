use lazy_static::lazy_static;
use regex::Regex;
use serenity::builder::CreateEmbed;
use serenity::framework::standard::{macros::command, Args, CommandResult};
use serenity::model::prelude::*;
use serenity::prelude::*;
use std::fmt::Write;
use std::time::Duration;

use crate::model::{
    sql::{GuildConfig, ModLogEntry},
    Confirmation,
};

enum CaseRange {
    /// A single case ID
    Single(u64),
    /// A range of inclusive case IDs
    Range { start: u64, end: u64 },
    /// A single latest case
    Latest,
    /// The latest number of cases
    LatestCount(u64),
}

#[command]
#[only_in("guild")]
#[required_permissions("BAN_MEMBERS")]
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
                .say(
                    &ctx.http,
                    "Please give a case ID, ID range, `latest`, or `latest~n`",
                )
                .await?;

            return Ok(());
        }
    };

    let attachments_str = msg
        .attachments
        .iter()
        .map(|a| a.url.as_str())
        .collect::<Vec<&str>>()
        .join("\n");

    let mut reason = args.rest();

    if reason.is_empty() && msg.attachments.is_empty() {
        msg.channel_id
            .say(&ctx.http, "Please give a reason or attachment")
            .await?;

        return Ok(());
    }

    if reason.is_empty() && !msg.attachments.is_empty() {
        reason = "View attachment(s)".into();
    }

    lazy_static! {
        // Limit to 19 chars max to prevent overflow, technically can have 20 digits but can still overflow
        // E.g.: u64 max: 18,446,744,073,709,551,615 -- Overflow with: 99,446,744,073,709,551,615
        static ref RE: Regex = Regex::new(r"(\d{1,19})\-(\d{1,19})").unwrap();
    }

    // Case ID range
    let case_range = if let Some(captures) = RE.captures(&range_str) {
        let start = captures.get(1).unwrap().as_str().parse::<u64>()?;
        let end = captures.get(2).unwrap().as_str().parse::<u64>()?;

        CaseRange::Range { start, end }

    // Single case ID
    } else if let Ok(num) = range_str.parse::<u64>() {
        CaseRange::Single(num)

    // Latest
    } else if range_str == "latest" {
        CaseRange::Latest

    // Latest n cases
    } else if range_str.starts_with("latest~") {
        let count = match range_str.trim_start_matches("latest~").parse::<u64>() {
            Ok(c) => c,
            Err(_) => {
                msg.channel_id
                        .say(
                            &ctx.http,
                            "Invalid number of latest cases, give a valid number after `latest~` (Example: `latest~3 for the latest 3 cases)",
                        )
                        .await?;

                return Ok(());
            }
        };

        CaseRange::LatestCount(count)
    } else {
        msg.channel_id
            .say(
                &ctx.http,
                "Invalid case, please give a case ID (2), ID range (1-4), `latest`, or `latest~n` (latest~3)",
            )
            .await?;

        return Ok(());
    };

    let mut entries = match case_range {
        CaseRange::Single(id) => ModLogEntry::get_range_entries(&ctx, guild_id, id, id).await?,
        CaseRange::Range { start, end } => {
            ModLogEntry::get_range_entries(&ctx, guild_id, start, end).await?
        }
        CaseRange::Latest => ModLogEntry::get_latest(&ctx, guild_id, 1).await?,
        CaseRange::LatestCount(count) => ModLogEntry::get_latest(&ctx, guild_id, count).await?,
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

    let num_with_reason =
        entries.iter().fold(
            0,
            |acc, entry| {
                if entry.reason.is_some() {
                    acc + 1
                } else {
                    acc
                }
            },
        );

    // Get confirmation if theres some cases already with reason se
    if num_with_reason > 0 {
        let mut desc = "There ".to_string();

        if num_with_reason == 1 {
            write!(desc, "is {}/{} case ", num_with_reason, entries.len())?;
        } else {
            write!(desc, "are {}/{} cases ", num_with_reason, entries.len())?;
        }

        writeln!(desc, "with a reason already set.")?;

        writeln!(desc)?;
        writeln!(desc, "React to choose and confirm your action")?;

        let mut opts = Vec::new();

        writeln!(desc, ":a: Overwrite all case reasons")?;
        opts.push((ReactionType::Unicode("🅰️".into()), "overwrite"));
        // If all cases have a reason, it's either overwrite all, or write none
        if num_with_reason != entries.len() {
            writeln!(desc, ":regional_indicator_s: Update only cases without a reason (skip cases already with a reason)")?;
            opts.push((ReactionType::Unicode("🇸".into()), "only_unset"));
        }

        writeln!(desc, ":x: Abort, don't update any case reasons")?;
        opts.push((ReactionType::Unicode("❌".into()), "abort"));

        let reason = reason.to_owned();

        let mut confirm = Confirmation::new(msg.author.id, move |e| {
            e.title("Warning");
            e.description(desc.clone());
            e.field("Reason", reason, false);
            e.footer(|f| f.text("Aborts in 1 minute"));

            e
        })
        .options(opts)
        .timeout(Duration::from_secs(60));

        match confirm.await_confirmation(ctx, msg.channel_id).await? {
            Some(r) if r == "overwrite" => {
                // Overwrite means just do nothing and continue
            }
            Some(r) if r == "only_unset" => {
                // Filter out cases with a reason set
                entries = entries.into_iter().filter(|e| e.reason.is_none()).collect();
            }
            Some(r) if r == "abort" => {
                msg.reply(ctx, "Aborted, no case reasons were updated.")
                    .await?;
                return Ok(());
            }
            Some(r) => {
                tracing::error!("Unhandled confirmation option: {}", r);
                msg.reply(ctx, "Error: Invalid options").await?;
                return Ok(());
            }
            None => {
                msg.reply(ctx, "No response after 1 minute, aborting.")
                    .await?;
                return Ok(());
            }
        }
    }

    // Since take ownership of entries in the iteration, just saving the length
    let num_entries = entries.len();

    // Needs to be updated when confirmation modifies entries
    let num_cases_str = if num_entries == 1 {
        "1 case".into()
    } else {
        format!("{} cases", num_entries)
    };

    let mut sent_msg = msg
        .channel_id
        .say(&ctx, format!("Updating {}...", num_cases_str))
        .await?;

    for mut entry in entries {
        let msg_id = match entry.msg_id {
            Some(id) => id,
            None => {
                tracing::warn!(?entry, "Missing msg_id for case");
                continue;
            }
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

        // Old description only, no fields embed
        if embed.fields.is_empty() {
            // Define here so we can use &str in the map below to not clone every line
            let new_reason_line = format!("**Reason:** {}", reason);

            // edit reason
            embed.description = embed.description.map(|d| {
                d.split('\n')
                    .map(|line| {
                        if line.starts_with("**Reason:**") {
                            &new_reason_line
                        } else {
                            line
                        }
                    })
                    .collect::<Vec<_>>()
                    .join("\n")
            });
        }

        // New embeds with fields
        if let Some(ref mut reason_field) = embed.fields.iter_mut().find(|f| f.name == "Reason") {
            reason_field.value = reason.to_string();
        }

        // Add attachments if there are any
        if !msg.attachments.is_empty() {
            let tip = if msg.attachments.len() == 1 {
                "(attachment shown below)"
            } else {
                "(first attachment shown below)"
            };

            if let Some(ref mut attachments_field) =
                embed.fields.iter_mut().find(|f| f.name == "Attachments")
            {
                attachments_field.value = format!("{}\n{}", attachments_str, tip);
            } else {
                embed.fields.push(EmbedField::new(
                    "Attachments",
                    format!("{}\n{}", attachments_str, tip),
                    false,
                ));
            }
        }

        if let Err(e) = message
            .edit(ctx, |m| {
                m.embed(|e| {
                    *e = CreateEmbed::from(embed);
                    e.author(|a| {
                        a.name(msg.author.tag());
                        a.icon_url(msg.author.face());

                        a
                    });

                    if let Some(attachment) = msg.attachments.first() {
                        e.image(&attachment.url);
                    }

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

    let mut s = format!(
        "Finished updating {} with reason: {}\n",
        num_cases_str, reason
    );

    writeln!(s, "Attachments: {}", attachments_str)?;

    sent_msg.edit(&ctx, |m| m.content(s)).await?;

    Ok(())
}
