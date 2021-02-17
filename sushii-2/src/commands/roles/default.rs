use serenity::framework::standard::{macros::command, CommandResult};
use serenity::model::prelude::*;
use serenity::prelude::*;

#[command]
#[required_permissions("MANAGE_GUILD")]
async fn default(ctx: &Context, msg: &Message) -> CommandResult {
    msg.channel_id
        .say(
            &ctx,
            "Available roles commands can be found here: <https://sushii.xyz/commands#roles>",
        )
        .await?;

    Ok(())
}
