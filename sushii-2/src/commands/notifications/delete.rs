use serenity::framework::standard::{macros::command, Args, CommandResult};
use serenity::model::prelude::*;
use serenity::prelude::*;

use crate::model::sql::*;

#[command]
async fn delete(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    // Delete message but don't return if error
    // Prevents other users from knowing notifications,
    // but without returning on error if sushii doesn't have perms
    let _ = msg.delete(ctx).await;

    let keyword = args.rest().trim();

    let noti = match Notification::user_notification(ctx, msg.author.id, keyword).await? {
        Some(noti) => noti,
        None => {
            msg.channel_id
                .say(&ctx, "Error: You don't have that keyword set")
                .await?;

            return Ok(());
        }
    };

    noti.delete(ctx).await?;

    msg.reply_mention(ctx, "Deleted keyword").await?;

    Ok(())
}
