use serenity::framework::standard::macros::hook;
use serenity::framework::standard::CommandError;
use serenity::framework::standard::DispatchError;
use serenity::model::prelude::*;
use serenity::prelude::*;

pub mod normal_message;
pub use self::normal_message::normal_message;

use crate::model::{
    sql::{BotStat, GuildConfig},
    Metrics,
};
use crate::DbPool;

#[hook]
pub async fn before(ctx: &Context, msg: &Message, cmd_name: &str) -> bool {
    let guild_conf = match GuildConfig::from_msg(&ctx, &msg).await.map_err(|e| {
        tracing::warn!("Failed to get guild config: {}", e);
    }) {
        Ok(Some(c)) => c,
        Ok(None) => return true, // in dms
        _ => return false,       // something failed
    };

    if let Some(channel) = guild_conf.role_channel {
        if msg.channel_id == channel as u64 {
            tracing::debug!(?msg, "Skipped command in role channel");
            return false;
        }
    }

    if let Some(disabled_channels) = guild_conf.disabled_channels {
        if disabled_channels.contains(&(msg.channel_id.0 as i64)) {
            return false;
        }
    }

    tracing::info!(author = %msg.author.tag(), %msg.content, "Running command {}", cmd_name);

    true
}

#[hook]
pub async fn dispatch_error(ctx: &Context, msg: &Message, error: DispatchError) {
    match error {
        DispatchError::NotEnoughArguments { min, given } => {
            let s = format!("This command needs {} arguments, only got {}.", min, given);

            let _ = msg.channel_id.say(&ctx, &s).await;
        }
        DispatchError::TooManyArguments { max, given } => {
            let s = format!("Max arguments allowed is {}, but got {}.", max, given);

            let _ = msg.channel_id.say(&ctx, &s).await;
        }
        DispatchError::LackingPermissions(permissions) => {
            let s = format!(
                "You do not have permissions to use this command, requires: `{}`",
                permissions
            );

            let _ = msg.channel_id.say(&ctx, &s).await;
        }
        DispatchError::Ratelimited(r) => {
            let _ = msg
                .channel_id
                .say(
                    &ctx,
                    format!(
                        "Error: Please wait {}s before using this command again",
                        r.rate_limit.as_secs()
                    ),
                )
                .await;
        }
        _ => tracing::warn!("Unhandled dispatch error: {:?}", error),
    }
}

#[hook]
pub async fn after(ctx: &Context, msg: &Message, cmd_name: &str, error: Result<(), CommandError>) {
    // Errors here are only from sushii errors, not user input errors
    if let Err(e) = error {
        tracing::error!(?msg, %e, "Error running command");

        // Downcast error
        // Disable sentry for now since it causes too many infinite loops
        // sentry::capture_error(&*e);

        let _ = msg
            .channel_id
            .say(
                &ctx,
                format!("Something went wrong while running this command :(\n{}", e),
            )
            .await;
    }

    {
        let metrics = ctx.data.read().await.get::<Metrics>().cloned().unwrap();
        let buf_count = {
            let mut buf_count = metrics.commands_executed_buffer.lock().await;
            *buf_count += 1;

            // Cloned
            buf_count.clone()
        };

        if buf_count >= 10 {
            // Reset before writing to db
            {
                let mut buf_count = metrics.commands_executed_buffer.lock().await;
                *buf_count = 0;
            }

            let pool = ctx.data.read().await.get::<DbPool>().cloned().unwrap();
            if let Err(e) = BotStat::inc(&pool, "bot", "commands_executed", buf_count as i64).await
            {
                tracing::error!("Failed to inc bot commands_executed stat: {}", e);
            }
        }
    }

    metrics::counter!("commands", 1, "command_name" => cmd_name.to_string());
}
