use anyhow::Result;
use chrono::{DateTime, Utc};
use std::collections::HashMap;
use sushii_model::model::sql::{Feed, FeedSubscription};
use twilight_http::api_error::{ApiError, ErrorCode};
use twilight_http::error::ErrorType;
use twilight_model::channel::embed::Embed;
use twilight_model::id::ChannelId;
use std::convert::TryFrom;
use vlive::{
    model::{recent_video::RecentVideo as VliveRecentVideo, video::PostDetail as VlivePostDetail},
    VLiveRequester,
};
use std::num::NonZeroU64;

use crate::embeddable::Embeddable;
use crate::model::context::Context;

pub async fn update_vlive(ctx: &Context, newer_than: DateTime<Utc>) -> Result<()> {
    // Only return Err() if fatal errors
    let new_vlives = get_new_vlive_items(ctx, newer_than).await?;

    if new_vlives.is_empty() {
        return Ok(());
    } else {
        tracing::debug!(?new_vlives, "New videos found");
    }

    // Get list of feed ids
    let feed_ids: Vec<_> = new_vlives
        .iter()
        .map(|item| format!("vlive:videos:{}", item.0.channel_code))
        .collect();

    // Fetch all subscriptions containing new video ids
    let matching_feeds = FeedSubscription::get_matching_vlive(&ctx.db_pool, &feed_ids).await?;

    // Create map feed_id -> Vec<subscriptions>
    let mut subscription_map = HashMap::new();
    for feed in matching_feeds {
        let entry = subscription_map
            .entry(feed.feed_id.clone())
            .or_insert_with(Vec::new);
        entry.push(feed);
    }

    for video in new_vlives {
        let feed_id = format!("vlive:videos:{}", video.0.channel_code);

        // subscriptions found
        if let Some(subscriptions) = subscription_map.get(&feed_id) {
            tracing::debug!(
                ?subscriptions,
                "Subscriptions found for {}",
                video.0.video_url()
            );

            let embed = match video.to_embed() {
                Ok(e) => e,
                Err(e) => {
                    tracing::error!(
                        "Failed to create embed for vlive video {}: {}",
                        video.0.video_url(),
                        e
                    );

                    continue;
                }
            };

            for sub in subscriptions {
                if let Err(e) = send_msg(ctx, sub, embed.clone()).await {
                    tracing::warn!(?e, "Failed to send feed message");
                }
            }
        } else {
            tracing::debug!(
                "No matching feeds for {} found, ignoring",
                video.0.video_url()
            );
        }
    }

    Ok(())
}

async fn send_msg(ctx: &Context, subscription: &FeedSubscription, embed: Embed) -> Result<()> {
    let embeds = [embed];

    let mut msg = ctx
        .http
        .create_message(ChannelId(NonZeroU64::try_from(subscription.channel_id as u64).unwrap()))
        .embeds(&embeds)?;

    let content = if let Some(role) = subscription.mention_role {
        format!("<@&{}>", role as u64)
    } else {
        "".into()
    };

    if subscription.mention_role.is_some() {
        msg = msg.content(&content)?;
    }

    // Send message
    let res = msg.exec().await;

    tracing::info!(
        "Sent vlive update to #{} in {}",
        subscription.channel_id as u64,
        subscription.guild_id as u64
    );

    match res {
        Err(e) => {
            if let ErrorType::Response {
                error: ApiError::General(e),
                ..
            } = e.kind() {
                tracing::warn!("Failed to send vlive update: {}", e);

                if e.code == ErrorCode::UnknownChannel || e.code == ErrorCode::Missingaccess {
                    tracing::warn!(?subscription, "Deleting feed subscription");
                    subscription.delete_pool(&ctx.db_pool).await?;
                }
            } else {
                return Err(e.into());
            }
        }
        Ok(_) => {}
    }

    Ok(())
}

async fn _update_rss(ctx: Context) -> Result<()> {
    let feeds = Feed::get_all_rss(&ctx.db_pool).await?;

    for _feed in feeds {
        /*
        match feed.metadata.0 {
            FeedMetadata::Rss(_meta) => {

            }
            _ => {}
            // VliveVideos(_)
            // VliveBoard(VliveBoardMetadata),
        }
        */
    }

    Ok(())
}

/// Only returns Err() if getting entire videos list fails
/// Will not return Err() if a single video fails
pub async fn get_new_vlive_items(
    ctx: &Context,
    newer_than: DateTime<Utc>,
) -> Result<Vec<(VliveRecentVideo, VlivePostDetail)>> {
    let videos = ctx.client.get_recent_videos(12, 1).await?;

    tracing::debug!(?videos, "Fetched recent videos");

    let mut new_items: Vec<(VliveRecentVideo, VlivePostDetail)> = Vec::new();

    for video in videos {
        let video_data = match ctx.client.get_video(video.video_seq).await {
            Ok(d) => d,
            Err(e) => {
                tracing::error!(video.video_seq, "Failed to get vlive video: {}", e);
                continue;
            }
        };

        let detail = match video_data.post_detail.get_detail() {
            Some(d) => d,
            None => {
                tracing::warn!(video.video_seq, "Video missing detail");
                continue;
            }
        };

        // Created at might be an old date, ie scheduled video
        let created_at = detail.official_video.created_at;
        // (might be wrong) thinking even if this is far in future, it won't
        // show up on front page unless it's live
        let on_air_start_at = detail.official_video.on_air_start_at;

        // Get older date
        let actual_start = if created_at > on_air_start_at {
            created_at
        } else {
            on_air_start_at
        };

        // Stop when videos are before newer_than. This relies on the fact that
        // get_recent_videos are sorted chronologically
        if actual_start < newer_than.naive_utc() {
            tracing::debug!(
                "Found old video {} @ {}, skipping rest",
                video.video_url(),
                detail.official_video.created_at
            );
            break;
        }

        new_items.push((video, detail.clone()));
    }

    Ok(new_items)
}
