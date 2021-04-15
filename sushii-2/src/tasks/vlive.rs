use serenity::model::prelude::*;
use serenity::prelude::*;
use std::collections::HashMap;

use crate::error::{Error, Result};
use crate::model::sql::*;
use sushii_feeds::{feed_request::feed_update_reply::Post, tonic, FeedServiceClient};

pub async fn check_new_vlives(
    ctx: &Context,
    tonic_client: FeedServiceClient<tonic::transport::channel::Channel>,
) -> Result<()> {
    let feeds = Feed::get_all_vlive(&ctx).await?;
    let feed_id_map: HashMap<&str, &Feed> = feeds
        .iter()
        .map(|feed| (feed.feed_id.as_str(), feed))
        .collect();

    let new_entries = sushii_feeds::get_new(tonic_client)
        .await
        .map_err(|e| Error::Sushii(format!("{}", e)))?;

    tracing::debug!("New feed items: {:?}", new_entries);

    for item in new_entries.items {
        let post = item.post.clone().unwrap();

        if FeedItem::from_id(&ctx, &item.feed_id, &post.id)
            .await?
            .is_some()
        {
            tracing::debug!(?post.id, "Already saved");
            // Skip if item already saved
            continue;
        }

        // Save the video to db to not fetch again
        if let Err(e) = FeedItem::new(&item.feed_id, &post.id).save(ctx).await {
            tracing::error!(?item, ?post, "Failed to save feed item: {}", e);
            continue;
        }

        tracing::debug!(?post.id, "Saved");

        // If active feed found for this channel
        if let Some(feed) = feed_id_map.get(&item.feed_id.as_str()) {
            let subscriptions = FeedSubscription::from_feed_id(&ctx, &item.feed_id).await?;
            tracing::debug!(?subscriptions, ?item.feed_id, "Subscriptions found");

            if subscriptions.is_empty() {
                feed.delete(&ctx).await?;
                tracing::debug!(?item.feed_id, "Feed deleted");
                continue;
            }

            for sub in subscriptions {
                if let Err(e) = send_msg(ctx, &post, sub).await {
                    tracing::warn!(?e, "Failed to send feed message");
                }
            }
        } else {
            tracing::debug!(?post.id, "No matching feed found, ignoring");
        }
    }

    Ok(())
}

#[tracing::instrument(skip(ctx))]
async fn send_msg(ctx: &Context, post: &Post, sub: FeedSubscription) -> Result<()> {
    let res = ChannelId(sub.channel_id as u64)
        .send_message(ctx, |m| {
            if let Some(mention_role) = sub.mention_role {
                m.content(format!("<@&{}>", mention_role as u64));
            }

            m.embed(|e| {
                if let Some(ref author) = post.author {
                    e.author(|a| {
                        a.name(&author.name);
                        a.icon_url(&author.icon);
                        a.url(&author.url);

                        a
                    });
                }

                e.title(&post.title);
                e.url(&post.url);
                e.description(&post.description);
                e.image(&post.thumbnail);
                e.colour(post.color);

                e.footer(|f| {
                    f.text("Powered by vlive.tv");
                    f.icon_url("https://i.imgur.com/NzGrmho.jpg");

                    f
                });

                e
            })
        })
        .await;

    match res {
        Ok(_) => Ok(()),
        Err(SerenityError::Http(e)) => {
            // Box cant be matched
            if let HttpError::UnsuccessfulRequest(ref e) = *e {
                tracing::warn!(?e, ?e.error, ?e.error.code, "Failed vlive feed send");

                // Unknown channel -- deleted channel so delete feed subscription
                // Or Missing permissions, delete it anyways whatever
                if e.error.code == 10003 || e.error.code == 50001 {
                    tracing::warn!(?sub, "Deleting feed subscription");
                    sub.delete(ctx).await?;
                }
            }

            Err(Error::Serenity(SerenityError::Http(e)))
        }
        Err(e) => Err(Error::Serenity(e)),
    }
}
