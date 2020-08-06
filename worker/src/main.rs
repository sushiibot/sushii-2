use std::{env, error::Error};
use tokio::stream::StreamExt;
use twilight::{
    cache::{
        twilight_cache_inmemory::config::{EventType, InMemoryConfigBuilder},
        InMemoryCache,
    },
    gateway::{
        cluster::{config::ShardScheme, Cluster, ClusterConfig},
        Event,
    },
    http::Client as HttpClient,
    model::gateway::GatewayIntents,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error + Send + Sync>> {
    let token = env::var("DISCORD_TOKEN")?;

    // This is also the default.
    let scheme = ShardScheme::Auto;

    let config = ClusterConfig::builder(&token)
        .shard_scheme(scheme)
        // Use intents to only listen to GUILD_MESSAGES events
        .intents(Some(
            GatewayIntents::GUILD_MESSAGES | GatewayIntents::DIRECT_MESSAGES,
        ))
        .build();

    // Start up the cluster
    let cluster = Cluster::new(config).await?;

    let cluster_spawn = cluster.clone();

    tokio::spawn(async move {
        cluster_spawn.up().await;
    });

    // The http client is seperate from the gateway,
    // so startup a new one
    let http = HttpClient::new(&token);

    // Since we only care about messages, make the cache only
    // cache message related events
    let cache_config = InMemoryConfigBuilder::new()
        .event_types(
            EventType::MESSAGE_CREATE
                | EventType::MESSAGE_DELETE
                | EventType::MESSAGE_DELETE_BULK
                | EventType::MESSAGE_UPDATE,
        )
        .build();
    let cache = InMemoryCache::from(cache_config);

    let mut events = cluster.events().await;
    // Startup an event loop for each event in the event stream
    while let Some(event) = events.next().await {
        // Update the cache
        cache.update(&event.1).await.expect("Cache failed, OhNoe");

        // Spawn a new task to handle the event
        tokio::spawn(handle_event(event, http.clone()));
    }

    Ok(())
}

async fn handle_event(
    event: (u64, Event),
    http: HttpClient,
) -> Result<(), Box<dyn Error + Send + Sync>> {
    match event {
        (_, Event::MessageCreate(msg)) if msg.content == "!ping" => {
            http.create_message(msg.channel_id)
                .content("Pong!")?
                .await?;
        }
        (id, Event::ShardConnected(_)) => {
            println!("Connected on shard {}", id);
        }
        _ => {}
    }

    Ok(())
}
