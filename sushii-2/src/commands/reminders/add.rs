use serenity::framework::standard::{macros::command, Args, CommandResult};
use serenity::model::prelude::*;
use serenity::prelude::*;

use crate::model::sql::*;

#[command]
#[only_in("guild")]
async fn add(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    let reminder_and_dur = args.rest().trim();

    if reminder_and_dur.is_empty() {
        msg.channel_id
            .say(&ctx, "Error: Please give a duration and reminder.")
            .await?;

        return Ok(());
    }
    // Save the actual notification
    let reminder = match Reminder::new(msg.author.id, reminder_and_dur) {
        Ok(r) => r.save(ctx).await?,
        Err(e) => {
            msg.reply(ctx, format!("Error: {}", e)).await?;

            return Ok(());
        }
    };

    if msg
        .author
        .dm(ctx, |m| {
            m.content(format!(
                "Ok! I'll remind you in {} (`{}` UTC)",
                reminder.get_human_duration(),
                reminder.expire_at.format("%Y-%m-%d %H:%M:%S")
            ))
        })
        .await
        .is_err()
    {
        msg.reply_mention(
            ctx,
            "Failed to send you a DM, I can't remind you :( \
            Check if you have them enabled!",
        )
        .await?;
    } else {
        msg.reply(ctx, ":mailbox_with_mail: Sent you a confirmation DM!")
            .await?;
    }

    Ok(())
}
