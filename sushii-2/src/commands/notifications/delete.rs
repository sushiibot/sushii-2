use serenity::framework::standard::{macros::command, Args, CommandResult};
use serenity::model::prelude::*;
use serenity::prelude::*;



#[command]
#[aliases("d", "del")]
#[only_in("guild")]
async fn delete(ctx: &Context, msg: &Message, _args: Args) -> CommandResult {
    msg.channel_id
        .say(&ctx.http, "Please use </notification delete:996259097441738764> now :)")
        .await?;

    return Ok(());
}
