use serenity::framework::standard::macros::hook;
use serenity::framework::standard::CommandError;
use serenity::framework::standard::DispatchError;
use serenity::model::prelude::*;
use serenity::prelude::*;

use crate::model::sql::{GuildConfig, GuildConfigDb};

#[hook]
pub async fn before(ctx: &Context, msg: &Message, cmd_name: &str) -> bool {
    let role_channel = GuildConfig::from_msg(&ctx, &msg)
        .await
        .ok()
        .unwrap_or(None)
        .and_then(|c| c.role_channel);

    if let Some(channel) = role_channel {
        if msg.channel_id == channel as u64 {
            tracing::debug!(?msg, "Skipped command in role channel");
            return false;
        }
    }

    tracing::info!(author = %msg.author.tag(), %msg.content, "Running command {}", cmd_name);

    true
}

#[hook]
pub async fn dispatch_error(context: &Context, msg: &Message, error: DispatchError) {
    match error {
        DispatchError::NotEnoughArguments { min, given } => {
            let s = format!("Need {} arguments, but only got {}.", min, given);

            let _ = msg.channel_id.say(&context, &s).await;
        }
        DispatchError::TooManyArguments { max, given } => {
            let s = format!("Max arguments allowed is {}, but got {}.", max, given);

            let _ = msg.channel_id.say(&context, &s).await;
        }
        _ => tracing::warn!("Unhandled dispatch error: {:?}", error),
    }
}

#[hook]
pub async fn after(_: &Context, msg: &Message, _: &str, error: Result<(), CommandError>) {
    // Don't respond to users here? can't determine error types and I don't want to
    // respond with all errors, possibly leaking extra info
    if let Err(e) = error {
        tracing::error!(?msg, %e, "Error running command");
    }
}
