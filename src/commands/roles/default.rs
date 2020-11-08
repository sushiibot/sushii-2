use serenity::framework::standard::{macros::command, CommandResult};
use serenity::model::prelude::*;
use serenity::prelude::*;

#[command]
async fn default(ctx: &Context, msg: &Message) -> CommandResult {
    msg.channel_id
        .say(
            &ctx,
            "Available roles commands can be found here: <https://2.sushii.xyz/commands#roles>",
        )
        .await?;

    Ok(())
}
