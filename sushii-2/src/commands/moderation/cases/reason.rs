use serenity::framework::standard::{macros::command, CommandResult};
use serenity::model::prelude::*;
use serenity::prelude::*;

#[command]
#[only_in("guild")]
#[required_permissions("BAN_MEMBERS")]
async fn reason(ctx: &Context, msg: &Message) -> CommandResult {
    msg.channel_id
        .say(&ctx.http, "Please use </reason:1070513842355974144> now :)")
        .await?;

    return Ok(());
}
