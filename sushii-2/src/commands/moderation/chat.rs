use serenity::framework::standard::{macros::command, Args, CommandResult};
use serenity::model::prelude::*;
use serenity::prelude::*;

#[command]
#[usage("[num messages]")]
#[aliases("p")]
#[required_permissions("MANAGE_GUILD")]
async fn prune(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let num_messages = match args.single::<u64>() {
        Ok(n) => n,
        Err(_) => {
            msg.channel_id
                .say(
                    &ctx.http,
                    "Error: Invalid number of messages, must be between 1 and 99 (inclusive)",
                )
                .await?;

            return Ok(());
        }
    };

    // Can delete 100, but we want to + 1 later so that the message invoking this
    // command isn't counted
    // also, needs to delete at least 2, but since we + 1, we can allow just 1
    if num_messages < 1 || num_messages > 99 {
        msg.channel_id
            .say(
                &ctx.http,
                "Error: Number of messages must be between 2 and 99 (inclusive)",
            )
            .await?;

        return Ok(());
    }

    let messages: Vec<MessageId> = msg
        .channel_id
        .messages(ctx, |r| r.limit(num_messages + 1))
        .await?
        .iter()
        .map(|m| m.id)
        .collect();

    msg.channel_id.delete_messages(ctx, messages).await?;

    Ok(())
}
