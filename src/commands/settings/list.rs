use serenity::framework::standard::{macros::command, CommandResult};
use serenity::model::prelude::*;
use serenity::prelude::*;

use crate::model::sql::guild::*;

#[command]
async fn list(ctx: &Context, msg: &Message) -> CommandResult {
    let conf = GuildConfig::from_msg_or_respond(&ctx, msg).await?;

    msg.channel_id
        .say(&ctx.http, format!("Guild settings:\n`{:#?}`", conf))
        .await?;

    Ok(())
}
