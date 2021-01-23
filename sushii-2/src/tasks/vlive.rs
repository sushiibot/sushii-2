use serenity::model::prelude::*;
use serenity::prelude::*;

use crate::error::{Result, Error};
use crate::model::sql::*;
use sushii_feeds::{tonic, FeedServiceClient};

pub async fn check_new_vlives(
    ctx: &Context,
    tonic_client: FeedServiceClient<tonic::transport::channel::Channel>,
) -> Result<()> {
    let new_entries = sushii_feeds::get_new(tonic_client).await
        .map_err(|e| Error::Sushii(format!("{}", e)))?;

    tracing::debug!("New feed items: {:?}", new_entries);

    for entry in new_entries.items {
        let sub = entry.subscription.unwrap();
        let post = entry.post.unwrap();


        ChannelId(sub.channel)
            .send_message(ctx, |m| {
                m.embed(|e| {
                    e.title(post.title)
                })
            })
            .await;
    }

    Ok(())
}
