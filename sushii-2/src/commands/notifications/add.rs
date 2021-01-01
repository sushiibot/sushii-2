use serenity::framework::standard::{macros::command, Args, CommandResult};
use serenity::model::prelude::*;
use serenity::prelude::*;

use crate::model::sql::*;

#[command]
async fn add(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    // Delete message but don't return if error
    // Prevents other users from knowing notifications,
    // but without returning on error if sushii doesn't have perms
    let _ = msg.delete(ctx).await;

    let global_and_keyword = args.rest().trim();

    if global_and_keyword.is_empty() {
        msg.channel_id
            .say(&ctx, "Error: Please give a keyword.  To add a global notification, add `global` in front (e.g. `global yourkeyword`).")
            .await?;

        return Ok(());
    }

    let is_global = global_and_keyword.starts_with("global ");

    // Remove global from keyword and whitespace
    let keyword = global_and_keyword.trim_start_matches("global ").trim();

    if Notification::user_notification(ctx, msg.author.id, keyword)
        .await?
        .is_some()
    {
        msg.channel_id
            .say(&ctx, "Error: You already have that keyword set")
            .await?;

        return Ok(());
    };

    let noti_guild_id = if is_global { msg.guild_id } else { None };

    // Save the actual notification
    Notification::new(msg.author.id, noti_guild_id, keyword)
        .save(ctx)
        .await?;

    let s = format!(
        "Added a {}notification with keyword: `{}`",
        if is_global { "global " } else { "" },
        keyword
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
