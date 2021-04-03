use serenity::framework::standard::{macros::command, CommandResult};
use serenity::model::prelude::*;
use serenity::prelude::*;

#[command]
async fn help(ctx: &Context, msg: &Message) -> CommandResult {
    msg.channel_id
        .say(
            &ctx,
            "You can find a list of commands here: <https://sushii.xyz/commands>\n\
            Join the support server if you still have questions: https://discord.gg/Bz5Q2WfuE7",
        )
        .await?;

    Ok(())
}
