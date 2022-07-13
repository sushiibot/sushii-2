use serenity::framework::standard::{macros::command, Args, CommandResult};
use serenity::model::prelude::*;
use serenity::prelude::*;

use crate::model::sql::*;

#[command]
#[aliases("a", "new")]
#[only_in("guild")]
async fn add(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    // Delete message but don't return if error
    // Prevents other users from knowing notifications,
    // but without returning on error if sushii doesn't have perms
    let _ = msg.delete(ctx).await;

    let guild_id = match msg.guild_id {
        Some(id) => id,
        None => {
            msg.channel_id.say(&ctx.http, "Error: No guild").await?;

            return Ok(());
        }
    };

    let keyword = args.rest().trim();

    if keyword.is_empty() {
        msg.channel_id
            .say(&ctx, "Error: Please give a keyword.")
            .await?;

        return Ok(());
    }

    // Already has keyword in single guild
    if Notification::user_notification(ctx, msg.author.id, guild_id, keyword)
        .await?
        .is_some()
    {
        msg.author
            .dm(&ctx, |m| {
                m.content("Error: You already have that keyword set in this server")
            })
            .await?;

        return Ok(());
    };

    // Save the actual notification
    let new_noti = Notification::new(msg.author.id, guild_id, keyword)
        .save(ctx)
        .await?;

    let guild_name = ctx
        .cache
        .guild_field(guild_id, |g| g.name.clone())
        .unwrap_or_else(|| "Unknown guild".into());

    // Keyword is converted to lowercase
    let s = format!(
        "Added a notification keyword `{}` in {}",
        new_noti.keyword, guild_name,
    );

    if msg.author.dm(ctx, |m| m.content(s)).await.is_err() {
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
