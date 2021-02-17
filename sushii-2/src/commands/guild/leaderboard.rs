use serenity::framework::standard::{macros::command, Args, CommandResult};
use serenity::model::prelude::*;
use serenity::prelude::*;

#[command]
#[aliases("toplevels", "topranks")]
#[only_in("guild")]
async fn leaderboard(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    let is_global = args.rest().trim() == "global";

    let guild_id = match msg.guild_id {
        Some(id) => id,
        None => {
            msg.channel_id.say(&ctx.http, "No guild found").await?;

            return Ok(());
        }
    };

    let url = if is_global {
        "https://sushii.xyz/leaderboard".into()
    } else {
        format!("https://sushii.xyz/leaderboard/{}", guild_id)
    };

    msg.reply(ctx, url).await?;

    Ok(())
}
