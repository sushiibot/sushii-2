use chrono::Utc;
use serenity::{model::prelude::*, prelude::*};
use std::fmt::Write;
use serenity::Error as SerenityError;
use serenity::http::error::Error as HttpError;

use crate::error::Result;
use crate::model::sql::*;

pub async fn message(ctx: &Context, msg: &Message) {
    if let Err(e) = _message(ctx, msg).await {
        tracing::error!(?msg, "Failed to run message handler: {}", e);
    }
}

async fn _message(ctx: &Context, msg: &Message) -> Result<()> {
    let guild_id = match msg.guild_id {
        Some(id) => id,
        None => return Ok(()),
    };

    if msg.author.bot {
        return Ok(());
    }

    let guild_conf = match GuildConfig::from_id(ctx, &guild_id).await? {
        Some(conf) => conf,
        None => return Ok(()),
    };

    // Don't log messages if message log isn't enabled or channel isn't set
    if !guild_conf.log_msg_enabled || guild_conf.log_msg.is_none() {
        return Ok(());
    }

    // Ignore role channel
    if let Some(channel) = guild_conf.role_channel {
        if msg.channel_id.0 == channel as u64 {
            return Ok(());
        }
    }

    let saved_msg = match SavedMessage::from_msg(msg) {
        Some(m) => m,
        None => return Ok(()),
    };

    // Save message to db
    saved_msg.save(ctx).await?;

    // Delete old messages past max # (100) to save per channel
    SavedMessage::prune_old(ctx, msg.channel_id).await?;

    Ok(())
}

pub async fn message_delete(
    ctx: &Context,
    channel_id: ChannelId,
    msg_id: MessageId,
    guild_id: Option<GuildId>,
) {
    if let Err(e) = _message_delete(ctx, channel_id, msg_id, guild_id).await {
        tracing::error!("Failed to run message_delete handler: {}", e);
    }
}

#[tracing::instrument(skip(ctx))]
async fn _message_delete(
    ctx: &Context,
    channel_id: ChannelId,
    msg_id: MessageId,
    guild_id: Option<GuildId>,
) -> Result<()> {
    let guild_id = match guild_id {
        Some(id) => id,
        None => return Ok(()),
    };

    let mut guild_conf = match GuildConfig::from_id(ctx, &guild_id).await? {
        Some(conf) => conf,
        None => return Ok(()),
    };

    // Don't log messages if message log isn't enabled or channel isn't set
    if !guild_conf.log_msg_enabled {
        return Ok(());
    }

    let log_msg_channel = match guild_conf.log_msg {
        Some(c) => c,
        None => return Ok(()),
    };

    // Ignore role channel
    if let Some(channel) = guild_conf.role_channel {
        if channel_id.0 == channel as u64 {
            return Ok(());
        }
    }

    let saved_msg = match SavedMessage::from_id(ctx, msg_id).await? {
        Some(msg) => msg,
        None => return Ok(()), // Not found
    };

    let mut attachments_s = String::new();

    for (i, attachment_url) in saved_msg
        .msg
        .attachments
        .iter()
        .map(|a| a.proxy_url.as_str())
        .enumerate()
    {
        let _ = write!(attachments_s, "[Attachment #{}]({})", i + 1, attachment_url);

        // Add comma if not last one
        if i < saved_msg.msg.attachments.len() - 1 {
            let _ = write!(attachments_s, ", ");
        }
    }

    let s = format!(
        "> {}\n\
        {}",
        saved_msg.content, attachments_s,
    );

    let now = Utc::now().naive_utc();

    let res = ChannelId(log_msg_channel as u64)
        .send_message(ctx, |m| {
            m.embed(|e| {
                e.description(format!(
                    "<@{}> in {}",
                    saved_msg.author_id as u64,
                    channel_id.mention()
                ));
                e.field("Message Deleted", s, false);

                e.footer(|f| {
                    f.text("Deleted at");

                    f
                });

                e.timestamp(now.format("%Y-%m-%dT%H:%M:%S").to_string());
                e.colour(0xe74c3c);

                e
            });

            m
        })
        .await;

    if let Err(SerenityError::Http(e)) = res {
        // Box cant be matched
        if let HttpError::UnsuccessfulRequest(e) = *e {
            tracing::warn!(?e, "HttpError::UnsuccessfulRequest");

            // Unknown channel -- deleted channel so just unset
            if e.error.code == 10003 {
                guild_conf.log_msg = None;
                guild_conf.save(ctx).await?;
            }
        }
    }

    Ok(())
}

pub async fn message_update(
    ctx: &Context,
    old_msg: &Option<Message>,
    new_msg: &Option<Message>,
    event: &MessageUpdateEvent,
) {
    if let Err(e) = _message_update(ctx, old_msg, new_msg, event).await {
        tracing::error!("Failed to run message_update handler: {}", e);
    }
}

#[tracing::instrument(skip(ctx))]
async fn _message_update(
    ctx: &Context,
    _old_msg: &Option<Message>,
    _new_msg: &Option<Message>,
    event: &MessageUpdateEvent,
) -> Result<()> {
    let new_content = match &event.content {
        Some(c) => c,
        None => return Ok(()),
    };

    let mut saved_msg = match SavedMessage::from_id(ctx, event.id).await? {
        Some(msg) => msg,
        None => return Ok(()), // Not found
    };

    let guild_conf = match GuildConfig::from_id(ctx, &GuildId(saved_msg.guild_id as u64)).await? {
        Some(conf) => conf,
        None => return Ok(()),
    };

    // Don't log messages if message log isn't enabled or channel isn't set
    if !guild_conf.log_msg_enabled {
        return Ok(());
    }

    let log_msg_channel = match guild_conf.log_msg {
        Some(c) => c,
        None => return Ok(()),
    };

    // Ignore role channel
    if let Some(channel) = guild_conf.role_channel {
        if saved_msg.channel_id == channel {
            return Ok(());
        }
    }

    let s = format!(
        "**Before:** {}\n\
        **+After:** {}",
        saved_msg.content, new_content,
    );

    let now = Utc::now().naive_utc();

    ChannelId(log_msg_channel as u64)
        .send_message(ctx, |m| {
            m.embed(|e| {
                e.description(format!(
                    "<@{}> in <#{}>",
                    saved_msg.author_id as u64, saved_msg.channel_id as u64
                ));
                e.field("Message Edited", s, false);

                e.footer(|f| {
                    f.text("Edited at");

                    f
                });

                e.timestamp(now.format("%Y-%m-%dT%H:%M:%S").to_string());
                e.colour(0x9b59b6);

                e
            });

            m
        })
        .await?;

    saved_msg.content = new_content.clone();

    // This doesn't update the actual serenity Message object in saved_msg.msg
    saved_msg.save(ctx).await?;

    Ok(())
}
