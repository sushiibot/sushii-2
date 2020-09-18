use serenity::prelude::*;
use tokio::{
    task,
    time::{self, Duration},
};

pub mod mute;

pub async fn start(ctx: &Context) {
    let ctx = ctx.clone();
    task::spawn(ten_seconds(ctx));
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
    }
}
