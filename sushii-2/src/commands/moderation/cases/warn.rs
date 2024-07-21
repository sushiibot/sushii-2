use serenity::framework::standard::{macros::command, Args, CommandResult};
use serenity::model::prelude::*;
use serenity::prelude::*;

#[command]
#[only_in("guild")]
#[required_permissions("BAN_MEMBERS")]
async fn warn(ctx: &Context, msg: &Message, _args: Args) -> CommandResult {
    msg.channel_id
        .say(&ctx.http, "Please use </warn:996259097202671646> now :)")
        .await?;

    return Ok(());
}
