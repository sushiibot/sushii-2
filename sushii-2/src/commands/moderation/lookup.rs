use serenity::framework::standard::{macros::command, Args, CommandResult};
use serenity::model::prelude::*;
use serenity::prelude::*;

#[command]
#[only_in("guild")]
#[required_permissions("BAN_MEMBERS")]
async fn lookup(ctx: &Context, msg: &Message, _args: Args) -> CommandResult {
    msg.channel_id
        .say(&ctx.http, "Please use </lookup:1117669298442338334> now :)")
        .await?;

    return Ok(());
}

#[command]
#[only_in("guild")]
#[required_permissions("BAN_MEMBERS")]
async fn optin(ctx: &Context, msg: &Message) -> CommandResult {
    msg.channel_id
    .say(&ctx.http, "Please use </settings lookup:1116541236480856104> now :)")
    .await?;

    Ok(())
}

#[command]
#[only_in("guild")]
#[required_permissions("BAN_MEMBERS")]
async fn optout(ctx: &Context, msg: &Message) -> CommandResult {
    msg.channel_id
    .say(&ctx.http, "Please use </settings lookup:1116541236480856104> now :)")
    .await?;

    Ok(())
}
