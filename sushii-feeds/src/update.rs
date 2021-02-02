use anyhow::Result;
use sushii_model::model::sql::{Feed, FeedMetadata};
use std::time::Duration;
use vlive::VLiveRequester;

use sushii_feeds::feed_request::feed_update_reply::{Author, FeedItem as GrpcFeedItem, Post};

use crate::model::context::Context;

async fn _update_rss(ctx: Context) -> Result<()> {
    let feeds = Feed::get_all_rss(&ctx.db_pool).await?;

    for feed in feeds {
        match feed.metadata.0 {
            FeedMetadata::Rss(_meta) => {

            }
            _ => {}
            // VliveVideos(_)
            // VliveBoard(VliveBoardMetadata),
        }
    }

    Ok(())
}

pub async fn update_vlive(ctx: Context) -> Result<Vec<GrpcFeedItem>> {
    let videos = ctx.client.get_recent_videos(12, 1).await?;
    let mut grpc_items = Vec::new();

    for video in videos {
        let video_data = ctx.client.get_video(video.video_seq).await;

        if let Err(ref e) = video_data {
            tracing::warn!("Failed to fetch video data: {}", e);
        }

        tracing::info!("video: {:?}", &video);

        let feed_id = format!("vlive:videos:{}", video.channel_code);

        // Author icon
        let icon = video_data
            .as_ref()
            .ok()
            .and_then(|d| d.channel.channel.channel_profile_image.clone())
            .unwrap_or_else(|| "https://i.imgur.com/NzGrmho.jpg".to_string());

        // If live or vod
        let title = if video.duration_secs.is_none() {
            format!("[LIVE] {}", video.title)
        } else {
            format!("[VOD] {}", video.title)
        };

        let description = if let Some(secs) = video.duration_secs {
            let d = Duration::from_secs(secs);
            format!("Duration: {}", humantime::format_duration(d))
        } else {
            "".to_string()
        };

        // Colour usually kinda dark and ugly
        /*
        let color = video_data
            .ok()
            .and_then(|d| d.channel.channel.representative_color)
            .and_then(|c| u32::from_str_radix(&c[1..], 16).ok())
            .unwrap_or(0x1ecfff); // Default bright teal vlive color
        */

        let grpc_post = Post {
            id: video.video_url(),
            title,
            author: Some(Author {
                name: video.channel_name.clone(),
                url: video.channel_url(),
                icon,
            }),
            description,
            thumbnail: video.thumbnail_url(),
            url: video.video_url(),
            color: 0x1ecfff,
        };

        let grpc_item = GrpcFeedItem {
            feed_id,
            post: Some(grpc_post),
        };
        grpc_items.push(grpc_item);
    }

    Ok(grpc_items)
}
