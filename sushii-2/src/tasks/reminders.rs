use serenity::model::prelude::*;
use serenity::prelude::*;

use crate::error::Result;
use crate::model::sql::*;

pub async fn check_expired_reminders(ctx: &Context) -> Result<()> {
    let reminders = Reminder::get_expired(ctx).await?;
    tracing::debug!("Found {} expired reminder entries", reminders.len());

    for reminder in reminders {
        if let Err(e) = remind_user(ctx, &reminder).await {
            tracing::error!(?reminder, "Failed to remind user: {}", e);
        }
    }

    Ok(())
}

pub async fn remind_user(ctx: &Context, reminder: &Reminder) -> Result<()> {
    ChannelId(reminder.channel_id as u64)
        .send_message(ctx, |m| {
            m.content(format!("<@{}>", reminder.user_id as u64));

            m.embed(|e| {
                e.title(format!(
                    "Reminder expired from {} ago",
                    reminder.get_human_duration()
                ));
                e.description(&reminder.description);

                e
            })
        })
        .await?;

    reminder.delete(ctx).await?;

    Ok(())
}
