use serenity::framework::standard::{macros::command, CommandResult};
use serenity::model::prelude::*;
use serenity::prelude::*;




#[command]
async fn list(ctx: &Context, msg: &Message) -> CommandResult {
    msg.channel_id
        .say(&ctx.http, "Please use </notification list:996259097441738764> now :)")
        .await?;

    return Ok(());
}
