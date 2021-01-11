use anyhow::anyhow;
use anyhow::Result;
use async_trait::async_trait;
use reqwest::Client;
use rss::Channel;
use sushii_model::model::sql::feed::{Feed, FeedKind, FeedMetadata};

pub async fn get_feed(client: Client, kind: FeedKind, feed: Feed) -> Result<Channel> {
    match kind {
        FeedKind::VliveChannelVideos | FeedKind::VliveChannelBoard => {
            return Err(anyhow!("Unsupported feed kind: {:?}", kind))
        }
        _ => {}
    }

    let data = match feed.metadata.0 {
        FeedMetadata::VliveBoard(_) | FeedMetadata::VliveVideos(_) => {
            return Err(anyhow!("Invalid feed metadata: {:?}", feed.metadata))
        }
        FeedMetadata::Rss(d) => d,
    };

    let content = client.get(&data.feed_url)
        .send()
        .await?
        .bytes()
        .await?;

    let channel = Channel::read_from(&content[..])?;

    Ok(channel)
}
