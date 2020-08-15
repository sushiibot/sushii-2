use crate::keys::*;
use serenity::framework::standard::{macros::command, Args, CommandResult};
use serenity::model::prelude::*;
use serenity::prelude::*;

use crate::utils::guild_config::{get_cached_guild_config_respond, upsert_config};

#[command]
#[only_in("guild")]
async fn settings(ctx: &Context, msg: &Message) -> CommandResult {
    let conf = get_cached_guild_config_respond(&ctx, msg).await?;

    msg.channel_id
        .say(&ctx.http, format!("Guild settings:\n`{:#?}`", conf))
        .await?;

    Ok(())
}

#[command]
#[only_in("guild")]
async fn prefix(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    let new_prefix = args.rest();
    let mut conf = get_cached_guild_config_respond(&ctx, msg).await?;

    if new_prefix.is_empty() {
        let current_prefix = match &conf.prefix {
            Some(p) => p.clone(),
            None => {
                let data = ctx.data.read().await;
                let sushii_cfg = data.get::<SushiiConfig>().unwrap();

                sushii_cfg.default_prefix.clone()
            }
        };

        msg.channel_id
            .say(
                &ctx.http,
                format!("The current guild prefix is: `{}`", current_prefix),
            )
            .await?;
        
        return Ok(());
    }

    conf.prefix.replace(new_prefix.to_string());

    upsert_config(&ctx, &conf).await?;

    msg.channel_id
        .say(&ctx.http, format!("Updated prefix to `{}`", new_prefix))
        .await?;

    Ok(())
}
