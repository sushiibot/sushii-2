use dotenv::dotenv;
use futures::pin_mut;
use metrics_exporter_prometheus::PrometheusBuilder;
use metrics_util::layers::{Layer, PrefixLayer};
use serde::Deserialize;
use std::pin::Pin;
use std::sync::Arc;
use tokio::sync::mpsc;
use tokio_stream::{Stream, StreamExt};
use tracing_subscriber::EnvFilter;
use twilight_http::Client;

use sushii_rules::{
    error::Result,
    model::{Event, RulesEngine},
};

mod gateway;

#[derive(Debug, Deserialize)]
pub struct RabbitMq {
    pub host: String,
    pub port: u64,
    pub username: String,
    pub password: String,
}

#[derive(Debug, Deserialize)]
pub struct Config {
    pub twilight_api_proxy_url: String,
    pub language_api_endpoint: String,

    pub database_url: String,

    pub rabbit: RabbitMq,

    #[serde(default)]
    pub redis: deadpool_redis::Config,
}

impl Config {
    pub fn from_env() -> Result<Self> {
        let mut cfg = config::Config::new();
        cfg.merge(config::Environment::new())?;
        Ok(cfg.try_into()?)
    }
}

fn start_metrics() {
    // Start metrics server
    let (recorder, exporter) = PrometheusBuilder::new()
        .build_with_exporter()
        .expect("Failed to build metrics recorder");

    let prefix = PrefixLayer::new("sushiirules_");
    let layered = prefix.layer(recorder);
    metrics::set_boxed_recorder(Box::new(layered)).expect("Failed to install recorder");

    // Spawn metrics hyper server in background
    tokio::spawn(async move {
        if let Err(e) = exporter.await {
            tracing::warn!("Metrics exporter error: {}", e);
        }
    });
}

#[tokio::main]
async fn main() -> Result<()> {
    dotenv().ok();

    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .init();

    let cfg = Config::from_env().expect("Failed to create config");

    tracing::info!("Config {:?}", &cfg);

    start_metrics();

    let pg_pool = sqlx::PgPool::connect(&cfg.database_url).await?;

    let redis_pool = cfg
        .redis
        .create_pool()
        .expect("Failed to create redis pool");

    let http = Client::builder()
        .proxy(cfg.twilight_api_proxy_url.clone(), true)
        .ratelimiter(None)
        .build();

    let current_user = http
        .current_user()
        .await
        .expect("Failed to fetch Discord current user, proxy API may be down");

    tracing::info!(
        "Connected as {}#{:0>4}",
        current_user.name,
        current_user.discriminator
    );

    // Trigger events from other rules, like counter updates or timers
    let (channel_tx, mut channel_rx) = mpsc::channel(32);

    let engine = RulesEngine::new(
        http,
        pg_pool,
        redis_pool,
        &cfg.language_api_endpoint,
        channel_tx,
    );

    let rabbit_stream = gateway::get_events(&cfg).await?;
    pin_mut!(rabbit_stream);

    let trigger_stream = Box::pin(async_stream::stream! {
        while let Some(event) = channel_rx.recv().await {
            yield Ok(event);
        }
    }) as Pin<Box<dyn Stream<Item = Result<Event>> + Send>>;

    // Merge both redis events and triggered events
    let mut rx = rabbit_stream.merge(trigger_stream);

    while let Some(event) = rx.next().await {
        let event = match event {
            Ok(e) => e,
            Err(e) => {
                tracing::error!("Error in event: {}", e);
                continue;
            }
        };

        if let Err(e) = engine.process_event(Arc::new(event)).await {
            tracing::error!("Failed to process event: {}", e);
        }
    }

    Ok(())
}
