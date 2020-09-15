use serenity::framework::standard::{macros::command, CommandResult};
use serenity::model::prelude::*;
use serenity::prelude::*;

#[command]
async fn ping(ctx: &Context, msg: &Message) -> CommandResult {
    msg.channel_id.say(&ctx.http, "Pong!").await?;

    Ok(())
}

#[command]
#[description("Gets the invite link for sushii")]
async fn invite(ctx: &Context, msg: &Message) -> CommandResult {
    // TODO: Pass invite link via config
    msg.channel_id.say(&ctx.http, "https://discord.com/api/oauth2/authorize?client_id=249784936318369793&permissions=388166&scope=bot").await?;

    Ok(())
}
