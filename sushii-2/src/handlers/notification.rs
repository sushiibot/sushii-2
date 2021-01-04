use serenity::{model::prelude::*, prelude::*};
use std::time::Instant;

use crate::error::Result;
use crate::model::sql::*;

pub async fn message(ctx: &Context, msg: &Message) {
    if let Err(e) = _message(ctx, msg).await {
        tracing::error!(?msg, "Failed to run message handler: {}", e);
    }
}

async fn _message(ctx: &Context, msg: &Message) -> Result<()> {
    // Notifications only in guilds
    let guild_id = match msg.guild_id {
        Some(id) => id,
        None => return Ok(()),
    };

    if msg.author.bot {
        return Ok(());
    }

    if msg.content.is_empty() {
        return Ok(());
    }

    let guild_name = msg
        .guild_field(ctx, |g| g.name.clone())
        .await
        .unwrap_or_else(|| "Unknown guild".into());

    // Get notifications from db with start/end times
    let start = Instant::now();
    let mut triggered_notis = Notification::get_matching(ctx, guild_id, &msg.content).await?;
    let delta = Instant::now() - start;

    metrics::histogram!("pg_notification_query_time", delta);
    metrics::counter!("pg_notification_query_count", triggered_notis.len() as u64);

    // Dedup notifications so that users only get 1 notification
    triggered_notis.sort_by(|a, b| a.user_id.cmp(&b.user_id));
    triggered_notis.dedup_by(|a, b| a.user_id == b.user_id);

    for noti in triggered_notis {
        // Don't notify self
        if noti.user_id as u64 == msg.author.id.0 {
            continue;
        }

        let channel = match ctx.cache.guild_channel(msg.channel_id).await {
            Some(channel) => channel,
            None => {
                tracing::warn!(?msg, "Notification trigger message channel not cached");

                // If this fails, then the other iterations will fail too
                return Ok(());
            }
        };

        // Returns Err if user isn't in guild
        match channel
            .permissions_for_user(&ctx.cache, noti.user_id as u64)
            .await
        {
            Ok(permissions) => {
                // User in guild but no permissions to read messages
                if !permissions.read_messages() {
                    continue;
                }
            }
            Err(_) => continue,
        }

        let s = format!(
            ":speech_left: {} mentioned `{}` in {} on {}\n> {}\n[Jump to message]({})",
            msg.author.tag(),
            noti.keyword,
            msg.channel_id.mention(),
            guild_name,
            msg.content,
            msg.link(),
        );

        let chan = match UserId(noti.user_id as u64).create_dm_channel(ctx).await {
            Ok(c) => c,
            Err(_) => continue,
        };

        let res = chan
            .send_message(ctx, |m| {
                m.embed(|e| {
                    e.description(s);
                    e.colour(0xf58b28);

                    e
                })
            })
            .await;

        if let Err(e) = res {
            tracing::warn!("Failed to send noti DM: {}", e);
        }
    }

    Ok(())
}
