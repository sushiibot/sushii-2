use serenity::{model::prelude::*, prelude::*};

use crate::error::Result;
use crate::model::sql::*;

pub async fn guild_member_addition(ctx: &Context, guild_id: &GuildId, member: &Member) {
    if let Err(e) = _guild_member_addition(&ctx, &guild_id, &member).await {
        tracing::error!("Failed to handle welcome guild_member_addition: {}", e);
    }
}

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

    let join_msg_replaced = join_msg
        .replace("<mention>", &member.user.mention())
        .replace("<username>", &member.user.name)
        .replace(
            "<server>",
            &guild_id.name(&ctx).await.unwrap_or_else(|| "".into()),
        );

    msg_channel.say(&ctx, join_msg_replaced).await?;

    Ok(())
}