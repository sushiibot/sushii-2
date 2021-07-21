use serenity::framework::standard::{macros::command, Args, CommandResult};
use serenity::model::prelude::*;
use serenity::prelude::*;

use crate::model::moderation::{ModActionExecutor, ModActionType};

#[command]
#[only_in("guild")]
#[required_permissions("BAN_MEMBERS")]
async fn note(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    let guild_id = match msg.guild_id {
        Some(id) => id,
        None => {
            msg.channel_id.say(&ctx.http, "No guild found").await?;

            return Ok(());
        }
    };

    ModActionExecutor::from_args(args, ModActionType::Note)
        .execute(&ctx, &msg, &guild_id)
        .await?;

    Ok(())
}
