use serenity::{model::prelude::*, prelude::*};

use crate::error::Result;
use crate::model::{sql::*, sushii_config::*};

pub async fn message(ctx: &Context, msg: &Message) {
    if let Err(e) = _message(ctx, msg).await {
        tracing::error!(?msg, "Failed to run message handler: {}", e);
    }
}

async fn _message(ctx: &Context, msg: &Message) -> Result<()> {
    if msg.author.bot {
        return Ok(());
    }

    let content_trim = msg.content.trim();

    // If not just a mention, without having to fetch id from cache and alloc
    // string on each message
    if content_trim.len() != 22 && content_trim.len() != 21 {
        return Ok(());
    }

    let bot_id = ctx.cache.current_user_id().await;

    // If mentioned **without** a command (since mention can be prefix)
    if msg.content.trim() != format!("<@!{}>", bot_id.0) {
        return Ok(());
    }

    let guild_conf = if let Some(ref id) = msg.guild_id {
        GuildConfig::from_id(ctx, id).await?
    } else {
        None
    };

    let sushii_conf = SushiiConfig::get(&ctx).await;

    let prefix = guild_conf
        .and_then(|c| c.prefix)
        .unwrap_or_else(|| sushii_conf.default_prefix.clone());

    let s = format!(
        "Hi! My prefix in this guild is `{}`. \
        You can also mention me ({}) as a prefix. \n\
        [The commands list can be found here](https://2.sushii.xyz/commands).\n\
        Still need help or have questions? \
        [Join the sushii support server](https://discord.gg/tQkb3GKVhP)",
        prefix,
        bot_id.mention()
    );

    msg.channel_id
        .send_message(&ctx.http, |m| {
            m.embed(|e| {
                e.description(&s);
                e.color(0xe67e22);

                e
            })
        })
        .await?;

    Ok(())
}
