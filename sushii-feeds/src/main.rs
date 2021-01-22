use anyhow::Result;
use darkredis::ConnectionPool;
use sqlx::postgres::PgPoolOptions;
use std::collections::HashMap;
use std::env;
use sushii_model::model::sql::{Feed, FeedItem, FeedMetadata, FeedSubscription};
use tokio::{
    task,
    time::{self, Duration},
};
use tracing_subscriber::filter::EnvFilter;
use vlive::VLiveRequester;

mod model;
use model::context::Context;

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

async fn update_vlive(ctx: Context) {
    if let Err(e) = _update_vlive(ctx).await {
        tracing::error!("Error updating vlive: {}", e);
    }
}

async fn _update_vlive(ctx: Context) -> Result<()> {
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

    for video in videos {
        // let video_data = ctx.client.get_video(video.video_seq).await?;
        tracing::info!("video: {:?}", &video);

        let feed_id = format!("vlive:videos:{}", video.channel_code);

        let feed_item = if let Some(item) =
            FeedItem::from_id(&ctx.db_pool, &feed_id, &video.video_url()).await?
        {
            item
        } else {
            FeedItem::new(
                format!("vlive:videos:{}", video.channel_code),
                video.video_url(),
            )
            .save(&ctx.db_pool)
            .await?
        };

        if let Some(feed) = feeds_map.get(&video.channel_code.as_str()) {
            let subscriptions = FeedSubscription::from_feed_id(&ctx.db_pool, &feed.feed_id).await?;

            // No subscriptions for this feed, delete it
            if subscriptions.is_empty() {
                feed.delete(&ctx.db_pool).await?;
                continue;
            }

            // Send notification to redis
            tracing::info!("New video: {:?}", video);
        }
    }

    Ok(())
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

#[tokio::main]
async fn main() -> Result<()> {
    dotenv::dotenv().ok();

    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .init();

    let db_url = env::var("DATABASE_URL").expect("Missing DATABASE_URL in environment");

    let db_pool = PgPoolOptions::new()
        .max_connections(2)
        .connect(&db_url)
        .await
        .expect("Failed to connect to database");

    let redis_addr = env::var("REDIS_HOST").expect("Missing REDIS_HOST in environment");
    let redis_pool = ConnectionPool::create(redis_addr, None, num_cpus::get()).await?;

    let ctx = Context::new(db_pool, redis_pool)?;

    run(ctx).await;

    Ok(())
}
