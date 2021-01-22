use anyhow::Result;
use sqlx::postgres::PgPoolOptions;
use std::collections::HashMap;
use std::env;
use std::net::SocketAddr;
use sushii_model::model::sql::{Feed, FeedItem, FeedMetadata, FeedSubscription};
use tokio::{
    task,
    time::{self, Duration},
};
use tonic::{transport::Server, Request, Response, Status};
use tracing_subscriber::filter::EnvFilter;
use vlive::VLiveRequester;

use crate::feed_request::{
    feed_update_reply::{Author, FeedItem as GrpcFeedItem, Post, Subscription},
    FeedUpdateReply,
};

use crate::model::context::Context;

async fn update_rss(ctx: Context) -> Result<()> {
    let feeds = Feed::get_all_rss(&ctx.db_pool).await?;

    for feed in feeds {
        match feed.metadata.0 {
            FeedMetadata::Rss(meta) => {

            }
            _ => {}
            // VliveVideos(_)
            // VliveBoard(VliveBoardMetadata),
        }
    }

    Ok(())
}

pub async fn update_vlive(ctx: Context) -> Result<Vec<GrpcFeedItem>> {
    let feeds = Feed::get_all_vlive(&ctx.db_pool).await?;

    // channel_code key
    let feeds_map: HashMap<&str, &Feed> = feeds
        .iter()
        .filter_map(|feed| {
            if let FeedMetadata::VliveVideos(ref meta) = feed.metadata.0 {
                Some((meta.channel.channel_code.as_str(), feed))
            } else {
                None
            }
        })
        .collect();

    let videos = ctx.client.get_recent_videos(12, 1).await?;

    tracing::debug!("videos: {:?}", videos);

    let mut grpc_items = Vec::new();

    for video in videos {
        // let video_data = ctx.client.get_video(video.video_seq).await?;
        tracing::info!("video: {:?}", &video);

        let feed_id = format!("vlive:videos:{}", video.channel_code);

        if FeedItem::from_id(&ctx.db_pool, &feed_id, &video.video_url())
            .await?
            .is_some()
        {
            // Skip if item already saved
            continue;
        } else {
            FeedItem::new(
                format!("vlive:videos:{}", video.channel_code),
                video.video_url(),
            )
            .save(&ctx.db_pool)
            .await?;
        };

        if let Some(feed) = feeds_map.get(&video.channel_code.as_str()) {
            let subscriptions = FeedSubscription::from_feed_id(&ctx.db_pool, &feed.feed_id).await?;

            // No subscriptions for this feed, delete it
            if subscriptions.is_empty() {
                feed.delete(&ctx.db_pool).await?;
                continue;
            }

            // New video found
            tracing::info!("New video: {:?}", video);
            let grpc_post = Post {
                title: video.title.clone(),
                author: Some(Author {
                    name: video.channel_name.clone(),
                    url: video.channel_url(),
                    icon: "".into(),
                }),
                description: format!("New video"),
                thumbnail: video.thumbnail_url.clone().unwrap_or_else(|| "".into()),
                url: video.video_url(),
            };

            for subscription in subscriptions {
                let grpc_item = GrpcFeedItem {
                    post: Some(grpc_post.clone()),
                    subscription: Some(Subscription {
                        channel: subscription.channel_id as u64,
                        role: subscription.mention_role.map_or(0, |id| id as u64),
                    }),
                };

                grpc_items.push(grpc_item);
            }
        }
    }

    Ok(grpc_items)
}

async fn run(ctx: Context) {
    let mut interval = time::interval(Duration::from_secs(30));

    tracing::info!("Starting update interval");

    loop {
        // Wait 10 seconds
        interval.tick().await;
        tracing::info!("Updating feeds");

        // Spawn API fetching on task
        task::spawn(update_rss(ctx.clone()));
        task::spawn(update_vlive(ctx.clone()));
    }
}
