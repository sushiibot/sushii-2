use chrono::Utc;
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

    let dm_msg = msg
        .author
        .dm(ctx, |m| {
            m.embed(|e| {
                e.title("Reminder set!");
                e.description(format!(
                    "Ok! I'll remind you <t:{0}:R> (<t:{0}>)",
                    reminder.expire_at.timestamp(),
                ));

                if !reminder.description.is_empty() {
                    e.field("Description", reminder.description, false);
                }

                e.footer(|f| f.text("Reminder set at"));
                e.timestamp(Utc::now().to_rfc3339());

                e
            });

            m
        })
        .await;

    if dm_msg.is_err() {
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
