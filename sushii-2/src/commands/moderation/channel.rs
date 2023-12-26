use serenity::framework::standard::{macros::command, Args, CommandResult};
use serenity::model::prelude::*;
use serenity::prelude::*;
use serenity::utils::parse_channel;

#[command]
#[aliases("sm")]
#[required_permissions("MANAGE_GUILD")]
async fn slowmode(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    msg.channel_id
    .say(&ctx.http, "Please use </slowmode:1016583377379397684> now :)")
    .await?;

    Ok(())
}
