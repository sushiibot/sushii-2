use serenity::framework::standard::macros::hook;
use serenity::framework::standard::CommandError;
use serenity::framework::standard::DispatchError;
use serenity::model::prelude::*;
use serenity::prelude::*;

#[hook]
pub async fn before(_ctx: &Context, msg: &Message, cmd_name: &str) -> bool {
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
