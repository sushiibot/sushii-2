use serenity::framework::standard::{macros::command, CommandResult};
use serenity::model::prelude::*;
use serenity::prelude::*;

#[command]
#[required_permissions("MANAGE_GUILD")]
async fn default(ctx: &Context, msg: &Message) -> CommandResult {
    msg.channel_id
        .say(
            &ctx,
            "Please use </rolemenu:1016166829032476793> now :)",
        )
        .await?;

    Ok(())
}
