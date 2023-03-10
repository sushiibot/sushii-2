use serenity::framework::standard::{macros::command, CommandResult};
use serenity::model::prelude::*;
use serenity::prelude::*;

#[command]
#[only_in("guild")]
#[required_permissions("BAN_MEMBERS")]
async fn ban(ctx: &Context, msg: &Message) -> CommandResult {
    msg.channel_id
        .say(&ctx.http, "Please use </ban:996259097202671643> now :)")
        .await?;

    return Ok(());
}

#[command]
#[only_in("guild")]
#[required_permissions("BAN_MEMBERS")]
async fn unban(ctx: &Context, msg: &Message) -> CommandResult {
    msg.channel_id
        .say(&ctx.http, "Please use </unban:1016166829032476792> now :)")
        .await?;

    return Ok(());
}
