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
    // Single failure_id for all of a users reminders, since they all should
    // fail if dms are disabled or server no longer shared
    let failure_id = format!("app_public.reminders:{}", reminder.user_id as u64);

    // First check if this failed too many times
    let failure = match Failure::from_id(ctx, &failure_id).await? {
        Some(f) => {
            // Too many attempts, delete and exit
            if f.exceeded_attempts() {
                tracing::info!(
                    ?reminder,
                    "Reminder fail count exceeded max attempts, deleting"
                );
                reminder.delete(ctx).await?;

                // Also delete failure since we don't need to keep track anymore
                f.delete(ctx).await?;

                return Ok(());
            }

            // Failed attempt before, skip until next attempt time
            // May accumulate for certain amount of time (25 failures)
            if !f.should_attempt() {
                return Ok(());
            }

            Some(f)
        }
        None => None,
    };

    // Try sending DM
    let res = UserId(reminder.user_id as u64)
        .create_dm_channel(ctx)
        .await?
        .send_message(ctx, |m| {
            m.embed(|e| {
                e.title(format!(
                    "Reminder expired from {} ago",
                    reminder.get_human_duration()
                ));
                e.description(&reminder.description);

                e
            })
        })
        .await;

    if res.is_ok() {
        // Success, delete reminder
        reminder.delete(ctx).await?;

        // Delete failure if any
        if let Some(failure) = failure {
            failure.delete(ctx).await?;
        }
    } else {
        // Failed, increment failure count
        // create a new one if first time erroring
        let mut failure = failure.unwrap_or_else(|| Failure::new(failure_id));
        failure.inc();
        failure = failure.save(ctx).await?;

        tracing::warn!(
            "Reminder DM failed (attempt {}), trying again at {}",
            failure.attempt_count,
            failure.next_attempt
        );
    }

    Ok(())
}
