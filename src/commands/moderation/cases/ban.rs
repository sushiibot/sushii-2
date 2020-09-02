use serenity::framework::standard::{macros::command, Args, CommandResult};
use serenity::model::prelude::*;
use serenity::prelude::*;

use crate::model::moderation::{ModActionExecutor, ModActionExecutorDb, ModActionType};

#[command]
#[only_in("guild")]
async fn ban(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    let guild_id = match msg.guild_id {
        Some(id) => id,
        None => {
            msg.channel_id.say(&ctx.http, "No guild found").await?;

            return Ok(());
        }
    };

    let bans = match guild_id.bans(&ctx.http).await {
        Ok(val) => val.iter().map(|x| x.user.id.0).collect::<Vec<u64>>(),
        Err(e) => {
            tracing::warn!("Failed to get guild bans: {}", e);

            Vec::new()
        }
    };

    ModActionExecutor::from_args(args, ModActionType::Ban)
        .exclude_users(bans)
        .execute(&ctx, &guild_id)
        .await?;

    Ok(())
}
