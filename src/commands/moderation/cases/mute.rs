use serenity::framework::standard::{macros::command, Args, CommandResult};
use serenity::model::prelude::*;
use serenity::prelude::*;

use crate::model::moderation::{ModActionExecutor, ModActionExecutorDb, ModActionType};
use crate::model::sql::*;

#[command]
#[only_in("guild")]
async fn mute(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    let guild_id = match msg.guild_id {
        Some(id) => id,
        None => {
            msg.channel_id.say(&ctx.http, "No guild found").await?;

            return Ok(());
        }
    };

    let conf = GuildConfig::from_msg_or_respond(&ctx, msg).await?;
    if conf.mute_role.is_none() {
        let _ = msg
            .channel_id
            .say(&ctx.http, "There is no mute command set");

        return Ok(());
    }

    ModActionExecutor::from_args(args, ModActionType::Mute)
        .execute(&ctx, &msg, &guild_id)
        .await?;

    Ok(())
}

#[command]
#[only_in("guild")]
async fn unmute(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    let guild_id = match msg.guild_id {
        Some(id) => id,
        None => {
            msg.channel_id.say(&ctx.http, "No guild found").await?;

            return Ok(());
        }
    };

    let conf = GuildConfig::from_msg_or_respond(&ctx, msg).await?;
    if conf.mute_role.is_none() {
        let _ = msg
            .channel_id
            .say(&ctx.http, "There is no mute command set");

        return Ok(());
    }

    ModActionExecutor::from_args(args, ModActionType::Unmute)
        .execute(&ctx, &msg, &guild_id)
        .await?;

    Ok(())
}
