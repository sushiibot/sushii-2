use serenity::model::prelude::*;
use serenity::prelude::*;
use serenity::framework::standard::macros::hook;

use crate::error::Result;
use crate::model::sql::*;
use crate::model::SushiiConfig;

#[hook]
pub async fn normal_message(ctx: &Context, msg: &Message) {
    if let Err(e) = _normal_message(ctx, msg).await {
        tracing::error!("Error handling normal message: {}", e);
    }
}

async fn _normal_message(ctx: &Context, msg: &Message) -> Result<()> {
    let guild_id = match msg.guild_id {
        Some(id) => id,
        None => return Ok(()),
    };

    let sushii_conf = SushiiConfig::get(&ctx).await;

    let prefix = GuildConfig::from_msg(&ctx, &msg).await?
        .and_then(|c| c.prefix)
        .unwrap_or_else(|| sushii_conf.default_prefix.clone());

    if !msg.content.starts_with(&prefix) {
        return Ok(());
    }

    let content_without_prefix = msg.content.trim_start_matches(&prefix);
    let split_pos = content_without_prefix.find(char::is_whitespace);

    let tag_name = if let Some(split_pos) = split_pos {
        // first part of split
        content_without_prefix.split_at(split_pos).0
    } else {
        // No whitespace so it's a single word
        content_without_prefix
    };

    let tag = match Tag::from_name(&ctx, tag_name, guild_id).await? {
        Some(t) => t,
        None => return Ok(()),
    };

    msg.channel_id.say(ctx, tag.content).await?;

    Ok(())
}
