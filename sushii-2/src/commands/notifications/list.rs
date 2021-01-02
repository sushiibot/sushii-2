use serenity::framework::standard::{macros::command, CommandResult};
use serenity::model::prelude::*;
use serenity::prelude::*;
use std::fmt::Write;

use crate::model::sql::*;

#[command]
async fn list(ctx: &Context, msg: &Message) -> CommandResult {
    let notis = Notification::user_notifications(ctx, msg.author.id).await?;

    if notis.is_empty() {
        msg.channel_id
            .say(&ctx, "Error: You have no notifications set")
            .await?;

        return Ok(());
    };

    let notis_global: Vec<_> = notis.iter().filter(|n| n.guild_id == 0).collect();
    let mut notis_guild: Vec<_> = notis.iter().filter(|n| n.guild_id != 0).collect();
    notis_guild.sort_by(|a, b| a.guild_id.cmp(&b.guild_id));

    let mut s = String::new();

    // Global notifications
    if !notis_global.is_empty() {
        writeln!(s, "**Global Notifications**")?;
    }

    for noti in notis_global {
        writeln!(s, "`{}`", noti.keyword)?;
    }

    // Add space between global and guild notifications
    if !notis_global.is_empty() && !notis_guild.is_empty() {
        writeln!(s)?;
    }

    // Guild notifications
    if !notis_guild.is_empty() {
        writeln!(s, "**Server Notifications**")?;
    }

    let mut last_guild_id = 0;

    for (i, noti) in notis_guild.iter().enumerate() {
        if last_guild_id != noti.guild_id {
            last_guild_id = noti.guild_id;

            let name = ctx
                .cache
                .guild_field(noti.guild_id as u64, |g| g.name.clone())
                .await
                .unwrap_or_else(|| "Unknown guild".into());

            if i != 0 {
                writeln!(s)?;
            }

            writeln!(s, "> **{}**", name)?;
        }

        writeln!(s, "> `{}`", noti.keyword)?;
    }

    let res = msg
        .author
        .dm(ctx, |m| {
            m.embed(|e| {
                e.title("Notification Keywords");
                e.description(s);

                e
            })
        })
        .await;

    if res.is_err() {
        // Not discord reply since original message is deleted
        msg.reply_mention(
            ctx,
            "Failed to send you a DM, I can't send you notifications :( \
            Check if you have them enabled!",
        )
        .await?;
    } else if !msg.is_private() {
        msg.channel_id
            .say(ctx, ":mailbox_with_mail: Sent a DM!")
            .await?;
    }

    Ok(())
}
