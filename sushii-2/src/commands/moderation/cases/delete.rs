use serenity::framework::standard::{macros::command, Args, CommandResult};
use serenity::model::prelude::*;
use serenity::prelude::*;

use sushii_model::model::sql::ModLogEntry;

#[command]
#[only_in("guild")]
#[required_permissions("BAN_MEMBERS")]
#[aliases("clearcase", "casedelete", "uncase")]
async fn deletecase(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    msg.channel_id
        .say(&ctx.http, "Please use </uncase:1075265021610823700> now :)")
        .await?;

    return Ok(());
}
