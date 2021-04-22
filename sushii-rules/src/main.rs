use anyhow::Result;
use dotenv::dotenv;
use redis::AsyncCommands;
use serde::{de::DeserializeSeed, Deserialize, Serialize};
use serde_json::Deserializer;
use tracing_subscriber::EnvFilter;
use twilight_http::Client;
use twilight_model::gateway::event::DispatchEvent;
use twilight_model::gateway::event::DispatchEventWithTypeDeserializer;

use sushii_rules::{model::RulesEngine, persistence::HardCodedStore};

#[derive(Debug, Deserialize)]
struct Config {
    /// Each worker is assigned to a cluster of shards as to prevent events
    /// being sent to multiple different workers. This isn't actually useful now
    /// since it's just a single cluster.
    pub cluster_id: u64,

    pub twilight_api_proxy_url: String,

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
    let gateway_event = de.deserialize(&mut json_deserializer)?;

    Ok(gateway_event)
}

#[tokio::main]
async fn main() -> Result<()> {
    dotenv().ok();

    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .init();

    let cfg = Config::from_env().expect("Failed to create config");

    tracing::info!("Config {:?}", &cfg);

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

    let current_user = http.current_user().await?;
    tracing::info!(
        "Connected as {}#{:0>4}",
        current_user.name,
        current_user.discriminator
    );

    let engine = RulesEngine::new(http, Box::new(HardCodedStore::new()));

    loop {
        let event = match get_event(&mut conn, &key).await {
            Ok(e) => e,
            Err(e) => {
                tracing::error!("Failed to get_event: {}", e);
                continue;
            }
        };

        if let Err(e) = engine.process_event(event) {
            tracing::error!("Failed to process event: {}", e);
        }
    }
}
