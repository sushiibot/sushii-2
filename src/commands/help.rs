use serenity::framework::standard::{macros::command, CommandResult};
use serenity::model::prelude::*;
use serenity::prelude::*;

#[command]
async fn help(ctx: &Context, msg: &Message) -> CommandResult {
    msg.channel_id.say(&ctx, "You can find a list of commands here: <https://2.sushii.xyz/commands>").await?;
    
    Ok(())
}
