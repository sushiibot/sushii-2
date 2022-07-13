use num_format::{Locale, ToFormattedString};
use serenity::{model::prelude::*, prelude::*};
use std::convert::TryFrom;

use crate::error::Result;
use crate::model::sql::*;

pub async fn guild_member_addition(ctx: &Context, guild_id: &GuildId, member: &Member) {
    if let Err(e) = _guild_member_addition(&ctx, &guild_id, &member).await {
        tracing::error!("Failed to handle welcome guild_member_addition: {}", e);
    }
}

#[tracing::instrument(skip(ctx))]
async fn _guild_member_addition(ctx: &Context, guild_id: &GuildId, member: &Member) -> Result<()> {
    let guild_conf = match GuildConfig::from_id(&ctx, &member.guild_id).await? {
        Some(c) => c,
        None => {
            tracing::error!(?member.guild_id, ?member, "No guild config found while handling mute guild_member_addition");
            return Ok(());
        }
    };

    if !guild_conf.join_msg_enabled {
        return Ok(());
    }

    let join_msg = match guild_conf.join_msg {
        Some(m) => m,
        None => return Ok(()),
    };

    let msg_channel = match guild_conf.msg_channel {
        Some(id) => ChannelId(id as u64),
        None => return Ok(()),
    };

    let member_number = match guild_id.to_guild_cached(&ctx) {
        Some(g) => g.member_count,
        None => 0,
    };

    let join_msg_replaced = join_msg
        .replace("<mention>", &member.user.mention().to_string())
        .replace("<username>", &member.user.name)
        .replace(
            "<member_number>",
            &member_number.to_formatted_string(&Locale::en),
        )
        .replace(
            "<server>",
            &guild_id.name(&ctx).unwrap_or_else(|| "".into()),
        );

    let msg = msg_channel.say(&ctx, join_msg_replaced).await?;

    // Convert string emoji to ReactionType to allow custom emojis
    if let Some(reaction) = guild_conf
        .join_react
        .and_then(|r| ReactionType::try_from(r).ok())
    {
        msg.react(&ctx, reaction).await?;
    }

    Ok(())
}
