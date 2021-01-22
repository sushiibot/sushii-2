use anyhow::anyhow;
use anyhow::Result;
use reqwest::Client;
use rss::Channel;
use std::collections::HashMap;
use strfmt::strfmt;
use sushii_model::model::sql::feed::{Feed, FeedKind, FeedMetadata};

pub mod model;

use crate::model::feeds::{FeedKindAttrs, FeedList};

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

    let content = client.get(&data.feed_url).send().await?.bytes().await?;

    let channel = Channel::read_from(&content[..])?;

    Ok(channel)
}

pub async fn list_available_feeds() -> Result<FeedList> {
    let s = tokio::fs::read("./feeds.json").await?;

    serde_json::from_slice(&s).map_err(Into::into)
}

/// Builds a saved Feed from a feed description (feeds.json) and required parameters
pub fn build_feed(feed_kind_attrs: FeedKindAttrs, params: HashMap<String, String>) -> Result<Feed> {
    match feed_kind_attrs.kind.as_str() {
        "vlive video" | "vlive board" => {
            return Err(anyhow!("Unsupported feed kind, only for RSS"));
        }
        _ => {
            let name = strfmt(&feed_kind_attrs.attributes.name, &params)?;
            let feed_url = strfmt(&feed_kind_attrs.attributes.feed_path, &params)?;
            let source_url = strfmt(&feed_kind_attrs.attributes.source_url, &params)?;

            let metadata = FeedMetadata::rss(name, feed_url, source_url);

            Ok(Feed::from_meta(metadata))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn deserializes_json() {
        let list = list_available_feeds().await.unwrap();

        let f = list.feeds.first().expect("First feed");

        assert!(list.feeds.len() > 0);
        assert_eq!(f.kind, "twitter".to_string());
    }
}
