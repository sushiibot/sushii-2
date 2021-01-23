use anyhow::Result;
use cached::proc_macro::cached;
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

use sushii_feeds::feed_request::{
    feed_update_reply::{Author, FeedItem as GrpcFeedItem, Post},
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
    let videos = ctx.client.get_recent_videos(12, 1).await?;
    let mut grpc_items = Vec::new();

    for video in videos {
        // let video_data = ctx.client.get_video(video.video_seq).await?;
        tracing::info!("video: {:?}", &video);

        let feed_id = format!("vlive:videos:{}", video.channel_code);

        let grpc_post = Post {
            id: video.video_url(),
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

        let grpc_item = GrpcFeedItem {
            feed_id,
            post: Some(grpc_post),
        };
        grpc_items.push(grpc_item);
    }

    Ok(grpc_items)
}
