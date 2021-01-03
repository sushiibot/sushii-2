use serenity::framework::standard::{macros::command, Args, CommandResult};
use serenity::model::prelude::*;
use serenity::prelude::*;
use std::time::Duration;

use crate::model::sql::*;
use crate::model::Confirmation;

#[command]
async fn add(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    // Delete message but don't return if error
    // Prevents other users from knowing notifications,
    // but without returning on error if sushii doesn't have perms
    let _ = msg.delete(ctx).await;

    let global_and_keyword = args.rest().trim();

    if global_and_keyword.is_empty() {
        msg.channel_id
            .say(&ctx, "Error: Please give a keyword. To add a global notification, add `global` in front (e.g. `global yourkeyword`).")
            .await?;

        return Ok(());
    }

    // If starts with global, or is in DM
    let is_global = global_and_keyword.starts_with("global ") || msg.guild_id.is_none();

    // Remove global from keyword and whitespace
    let keyword = global_and_keyword.trim_start_matches("global ").trim();

    if let Some(existing_noti) =
        Notification::user_notification(ctx, msg.author.id, keyword).await?
    {
        // Want to add global, but already is global
        if is_global && existing_noti.guild_id == 0 {
            msg.author
                .dm(&ctx, |m| {
                    m.content("Error: You already have that global keyword set")
                })
                .await?;

            return Ok(());
        }

        // Want to add local, but already has in this guild
        if !is_global
            && msg
                .guild_id
                .map_or(false, |id| id == existing_noti.guild_id as u64)
        {
            msg.author
                .dm(&ctx, |m| {
                    m.content("Error: You already have that keyword set in this server")
                })
                .await?;

            return Ok(());
        }

        let s = if is_global {
            format!(
                "Do you want to change the keyword {} to a **global** notification?",
                keyword
            )
        } else {
            if let Some(guild_id) = msg.guild_id {
                let guild_name = ctx
                    .cache
                    .guild_field(guild_id, |g| g.name.clone())
                    .await
                    .unwrap_or_else(|| "Unknown guild".into());

                format!(
                    "Do you want to change the global keyword {} to a **server** notification in {}?",
                    keyword, guild_name
                )
            } else {
                format!(
                    "Do you want to change the global keyword {} to a **server** notification?",
                    keyword
                )
            }
        };

        let mut opts = Vec::new();
        opts.push((ReactionType::Unicode("✅".into()), "yes"));
        opts.push((ReactionType::Unicode("❌".into()), "no"));

        let dm_chan = msg.author.create_dm_channel(ctx).await?.id;

        let mut confirm = Confirmation::new(msg.author.id, move |e| {
            e.title("Keyword already exists");
            e.description(s);
            e.footer(|f| f.text("Aborts in 1 minute"));

            e
        })
        .options(opts)
        .timeout(Duration::from_secs(60));

        match confirm.await_confirmation(ctx, dm_chan).await? {
            Some(r) if r == "yes" => {
                // Delete old one if yes
                existing_noti.delete(ctx).await?;
            }
            Some(r) if r == "no" => {
                dm_chan.say(ctx, "Alright! No changes are made.").await?;

                return Ok(());
            }
            Some(r) => {
                tracing::error!("Unhandled confirmation option: {}", r);

                dm_chan.say(ctx, "Error: Invalid options").await?;

                return Ok(());
            }
            None => {
                dm_chan
                    .say(ctx, "No response after 1 minute, aborting.")
                    .await?;

                return Ok(());
            }
        }
    };

    let noti_guild_id = if is_global { None } else { msg.guild_id };

    // Save the actual notification
    let new_noti = Notification::new(msg.author.id, noti_guild_id, keyword)
        .save(ctx)
        .await?;

    // Keyword is converted to lowercase
    let s = format!(
        "Added a {}notification keyword: `{}`",
        if is_global { "global " } else { "" },
        new_noti.keyword
    );

    if let Err(_) = msg.author.dm(ctx, |m| m.content(s)).await {
        // Not discord reply since original message is deleted
        msg.reply_mention(
            ctx,
            "Failed to send you a DM, I can't send you notifications :( \
            Check if you have them enabled!",
        )
        .await?;
    }

    Ok(())
}
