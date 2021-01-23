use serenity::model::prelude::*;
use serenity::prelude::*;

use crate::error::{Error, Result};
use crate::model::sql::*;
use std::collections::HashMap;
use sushii_feeds::{tonic, FeedServiceClient};

pub async fn check_new_vlives(
    ctx: &Context,
    tonic_client: FeedServiceClient<tonic::transport::channel::Channel>,
) -> Result<()> {
    let feeds = Feed::get_all_vlive(&ctx).await?;
    let feed_id_map: HashMap<&str, &Feed> =
        feeds.iter().map(|feed| (feed.feed_id.as_str(), feed)).collect();

    let new_entries = sushii_feeds::get_new(tonic_client)
        .await
        .map_err(|e| Error::Sushii(format!("{}", e)))?;

    tracing::debug!("New feed items: {:?}", new_entries);

    for item in new_entries.items {
        let post = item.post.unwrap();

        if let Some(feed) = feed_id_map.get(&item.feed_id.as_str()) {
            if FeedItem::from_id(&ctx, &item.feed_id, &post.url)
                .await?
                .is_some()
            {
                // Skip if item already saved
                continue;
            }

            // Save the video to db to not fetch again
            FeedItem::new(&item.feed_id, &post.id).save(ctx).await?;

            let subscriptions = FeedSubscription::from_feed_id(&ctx, &item.feed_id).await?;

            if subscriptions.is_empty() {
                feed.delete(&ctx).await?;
                continue;
            }

            for sub in subscriptions {
                if let Err(e) = ChannelId(sub.channel_id as u64)
                    .send_message(ctx, |m| m.embed(|e| e.title(&post.title)))
                    .await
                {
                    tracing::warn!("Failed to send feed message: {}", e);
                    // TODO: Delete this subscription if fails too many times
                }
            }
        }
    }

    Ok(())
}
