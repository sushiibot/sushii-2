use anyhow::Result;
use dotenv::dotenv;
use metrics_exporter_prometheus::PrometheusBuilder;
use metrics_util::layers::{Layer, PrefixLayer};
use redis::AsyncCommands;
use serde::{de::DeserializeSeed, Deserialize, Serialize};
use serde_json::Deserializer;
use std::pin::Pin;
use std::sync::Arc;
use tokio::sync::mpsc;
use tokio_stream::{Stream, StreamExt};
use tracing_subscriber::EnvFilter;
use twilight_http::Client;
use twilight_model::gateway::event::DispatchEvent;
use twilight_model::gateway::event::DispatchEventWithTypeDeserializer;

use sushii_rules::{
    error::Error,
    model::{Event, RulesEngine},
    persistence::HardCodedStore,
};

#[derive(Debug, Deserialize)]
struct Config {
    /// Each worker is assigned to a cluster of shards as to prevent events
    /// being sent to multiple different workers. This isn't actually useful now
    /// since it's just a single cluster.
    pub cluster_id: u64,

    pub twilight_api_proxy_url: String,
    pub language_api_endpoint: String,

    pub database_url: String,

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

#[derive(Serialize, Deserialize)]
struct RedisEvent {
    pub name: String,
    pub payload: String,
}

async fn get_event(
    conn: &mut deadpool_redis::ConnectionWrapper,
    key: &str,
) -> Result<DispatchEvent> {
    let popped: Vec<String> = conn.blpop::<&str, Vec<String>>(&key, 0).await?;
    // https://redis.io/commands/blpop
    // A two-element multi-bulk with the first element being the name of the key
    // where an element was popped and the second element being the value of the
    // popped element.
    let event: RedisEvent = serde_json::from_str(&popped[1])?;

    let mut json_deserializer = Deserializer::from_str(&event.payload);

    let de = DispatchEventWithTypeDeserializer::new(&event.name);
    let gateway_event = de
        .deserialize(&mut json_deserializer)
        .map_err(|e| Error::EventDeserialize(event.name, e))?;

    Ok(gateway_event)
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

    let pool = cfg
        .redis
        .create_pool()
        .expect("Failed to create redis pool");

    let mut conn = pool.get().await.unwrap();
    let key = format!("events:{}", cfg.cluster_id);

    tracing::info!("Watching events on list `{}`", &key);

    let http = Client::builder()
        .proxy(cfg.twilight_api_proxy_url, true)
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
        Box::new(HardCodedStore::new()),
        &cfg.language_api_endpoint,
        channel_tx,
    );

    let redis_stream = Box::pin(async_stream::stream! {
        loop {
            match get_event(&mut conn, &key).await {
                Ok(e) => yield Event::Twilight(e),
                Err(e) => {
                    tracing::error!("Failed get_event: {}", e);
                    continue;
                }
            }
        }
    }) as Pin<Box<dyn Stream<Item = Event> + Send>>;

    let trigger_stream = Box::pin(async_stream::stream! {
        while let Some(event) = channel_rx.recv().await {
            yield event;
        }
    }) as Pin<Box<dyn Stream<Item = Event> + Send>>;

    // Merge both redis events and triggered events
    let mut rx = redis_stream.merge(trigger_stream);

    while let Some(event) = rx.next().await {
        if let Err(e) = engine.process_event(Arc::new(event)) {
            tracing::error!("Failed to process event: {}", e);
        }
    }

    Ok(())
}
