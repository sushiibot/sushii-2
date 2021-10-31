use anyhow::Result;
use chrono::Utc;
use sqlx::postgres::PgPoolOptions;
use std::env;
use tokio::time::{self, Duration};
use tracing_subscriber::filter::EnvFilter;
use twilight_http::Client;
use std::sync::Arc;

mod embeddable;
mod model;
mod update;
use model::context::Context;

async fn run(ctx: Context) {
    let interval_duration = Duration::from_secs(60);
    tracing::info!(
        "Starting feed loop, update interval: {:?}",
        &interval_duration
    );
    let mut interval = time::interval(interval_duration);

    // Keep track of which feed items are considered "new"
    // Any items older than this date will be ignored
    let mut newer_than = Utc::now();

    loop {
        interval.tick().await;

        // Now before update_feeds runs since there might be new items that are
        // created during the update
        let now = Utc::now();
        tracing::debug!(?now, "Tick");

        // Actually update the feeds and send Discord feed messages
        if let Err(e) = update::update_vlive(&ctx, newer_than).await {
            tracing::error!("Failed to update vlive feeds: {}", e);
        }

        tracing::debug!("Finished update");

        // Set new time after the update
        newer_than = now;
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    dotenv::dotenv().ok();
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .init();

    // Vars
    let db_url = env::var("DATABASE_URL").expect("Missing DATABASE_URL in environment");
    let proxy_url =
        env::var("TWILIGHT_API_PROXY_URL").expect("Missing TWILIGHT_API_PROXY_URL in environment");

    // Database
    let db_pool = PgPoolOptions::new()
        .max_connections(2)
        .connect(&db_url)
        .await
        .expect("Failed to connect to database");

    // Twilight http client
    let http = Arc::new(Client::builder()
        .proxy(proxy_url, true)
        .ratelimiter(None)
        .build());

    let current_user = http.current_user().exec().await?.model().await?;
    tracing::info!(
        "Connected as {}#{:0>4}",
        current_user.name,
        current_user.discriminator
    );

    let ctx = Context::new(http, db_pool)?;

    // Finally run update loop
    // This should never return unless process is killed
    run(ctx).await;

    Ok(())
}
