use serenity::prelude::*;
use std::sync::Once;
use tokio::{
    task,
    time::{self, Duration},
};

static START: Once = Once::new();

mod mute;
mod reminders;

pub async fn start(ctx: &Context) {
    START.call_once(|| {
        task::spawn(ten_seconds(ctx.clone()));
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
