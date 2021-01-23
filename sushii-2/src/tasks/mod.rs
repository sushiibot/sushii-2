use serenity::prelude::*;
use std::sync::Once;
use sushii_feeds::FeedServiceClient;
use tokio::{
    task,
    time::{self, Duration},
};

use crate::model::SushiiConfig;

static START: Once = Once::new();

mod mute;
mod reminders;
mod vlive;

pub async fn start(ctx: &Context) {
    START.call_once(|| {
        task::spawn(ten_seconds(ctx.clone()));
        task::spawn(thirty_seconds(ctx.clone()));
    });
}

async fn ten_seconds(ctx: Context) {
    let mut interval = time::interval(Duration::from_secs(10));

    loop {
        // Wait 10 seconds
        interval.tick().await;
        tracing::debug!("Checking for expired mute entries...");

        if let Err(e) = mute::check_pending_unmutes(&ctx).await {
            tracing::error!("Failed checking pending unmutes: {}", e);
        }

        if let Err(e) = reminders::check_expired_reminders(&ctx).await {
            tracing::error!("Failed checking expired reminders: {}", e);
        }
    }
}

async fn thirty_seconds(ctx: Context) {
    let mut interval = time::interval(Duration::from_secs(30));

    let cfg = SushiiConfig::get(&ctx).await;

    let tonic_client = match FeedServiceClient::connect(cfg.feed_server_url.clone()).await {
        Ok(c) => c,
        Err(e) => {
            tracing::error!("Failed to connect to feed server: {}", e);
            return;
        }
    };

    loop {
        interval.tick().await;
        tracing::debug!("Checking for new vlives...");

        if let Err(e) = vlive::check_new_vlives(&ctx, tonic_client.clone()).await {
            tracing::error!("Failed checking for new vlives: {}", e);
        }
    }
}
