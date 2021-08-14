use serenity::framework::standard::{macros::command, CommandResult};
use serenity::model::prelude::*;
use serenity::prelude::*;
use std::fmt::Write;

use crate::model::sql::*;

#[command]
async fn list(ctx: &Context, msg: &Message) -> CommandResult {
    let mut reminders = Reminder::user_reminders(ctx, msg.author.id).await?;

    if reminders.is_empty() {
        msg.channel_id
            .say(&ctx, "Error: You have no reminders set")
            .await?;

        return Ok(());
    };

    // Sort by expire time
    reminders.sort_by(|a, b| a.expire_at.cmp(&b.expire_at));

    let mut s = String::new();

    writeln!(s, "`Expire Date` | `Time Left` | `Description`")?;

    for reminder in &reminders {
        writeln!(
            s,
            "<t:{}> | <t:{}:R> | {}",
            reminder.expire_at.timestamp(),
            reminder.expire_at.timestamp(),
            reminder.description
        )?;
    }

    msg.channel_id
        .send_message(ctx, |m| {
            m.embed(|e| {
                e.title(format!("Your Reminders ({} Total)", reminders.len()));
                e.description(s);
                e.footer(|f| f.text("Date format: YYYY-MM-DD • Times in UTC"));

                e
            })
        })
        .await?;

    Ok(())
}
