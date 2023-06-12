use serenity::framework::standard::{macros::command, Args, CommandResult};
use serenity::model::prelude::*;
use serenity::prelude::*;

#[command]
#[only_in("guild")]
#[required_permissions("BAN_MEMBERS")]
async fn note(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    msg.channel_id
        .say(&ctx.http, "Please use </note:1017270490634670110> now :)")
        .await?;

return Ok(());
}
