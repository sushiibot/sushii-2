use serenity::prelude::*;
use std::sync::Once;
use tokio::{
    task,
    time::{self, Duration},
};

static START: Once = Once::new();

mod mute;
mod reminders;
mod stats;

pub async fn start(ctx: &Context) {
    START.call_once(|| {
        task::spawn(ten_seconds(ctx.clone()));
        task::spawn(five_minutes(ctx.clone()));
    });
}

async fn ten_seconds(ctx: Context) {
    let mut interval = time::interval(Duration::from_secs(10));

    loop {
        // Wait 10 seconds
        interval.tick().await;
        tracing::debug!("Checking for expired mute entries...");

        // if let Err(e) = mute::check_pending_unmutes(&ctx).await {
        //     tracing::error!("Failed checking pending unmutes: {}", e);
        // }

        if let Err(e) = reminders::check_expired_reminders(&ctx).await {
            tracing::error!("Failed checking expired reminders: {}", e);
        }
    }
}

async fn five_minutes(ctx: Context) {
    let mut interval = time::interval(Duration::from_secs(60 * 5));

    loop {
        // Wait 5 minutes
        interval.tick().await;
        tracing::debug!("Updating bot stats...");

        if let Err(e) = stats::update_stats(&ctx).await {
            tracing::error!("Failed updating stats: {}", e);
        }
    }
}
