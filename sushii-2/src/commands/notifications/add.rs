use serenity::framework::standard::{macros::command, Args, CommandResult};
use serenity::model::prelude::*;
use serenity::prelude::*;



#[command]
#[aliases("a", "new")]
#[only_in("guild")]
async fn add(ctx: &Context, msg: &Message, _args: Args) -> CommandResult {
    msg.channel_id
        .say(&ctx.http, "Please use </notification add:996259097441738764> now :)")
        .await?;

    return Ok(());
}
