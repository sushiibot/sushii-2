use serenity::framework::standard::{Args, macros::command, CommandResult};
use serenity::model::prelude::*;
use serenity::prelude::*;

#[command]
#[num_args(1)]
#[usage("[num messages]")]
#[only_in("guild")]
async fn prune(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let num_messages = args
        .single::<u64>()
        .map_err(|_| "Invalid input, please give a number".to_string())?;

    if num_messages < 2 || num_messages > 100 {
        return Err("Number of messages must be between 2 and 100 (inclusive)".into());
    }

    let messages: Vec<MessageId> = msg.channel_id
        .messages(&ctx.http, |r| r.limit(num_messages))
        .await?
        .iter()
        .map(|m| m.id)
        .collect();

    msg.channel_id.delete_messages(&ctx.http, messages).await?;

    Ok(())
}
