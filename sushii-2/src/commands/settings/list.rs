use serenity::framework::standard::{macros::command, CommandResult};
use serenity::model::prelude::*;
use serenity::prelude::*;

use crate::model::sql::*;

#[command]
#[required_permissions("MANAGE_GUILD")]
async fn list(ctx: &Context, msg: &Message) -> CommandResult {
    let conf = GuildConfig::from_msg_or_respond(&ctx, msg).await?;

    let s = format!(
        "<:online:316354435745972244> Enabled\n<:offline:316354467031416832> Disabled\n\n{}",
        conf
    );

    msg.channel_id
        .send_message(&ctx.http, |m| {
            m.embed(|e| {
                e.title("Guild Settings");
                e.color(0xe67e22);

                e.description(s);

                e
            })
        })
        .await?;

    Ok(())
}
