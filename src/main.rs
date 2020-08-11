use sqlx::postgres::PgPoolOptions;
use std::sync::Arc;
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

mod error;
mod handlers;
mod model;
mod utils;

use crate::error::Result;

#[tokio::main]
async fn main() -> Result<()> {
    // install global subscriber configured based on RUST_LOG envvar.
    tracing_subscriber::fmt().init();

    let sushii_conf = model::sushii_config::SushiiConfig::new_from_env().expect("failed to make config");

    // The http client is seperate from the gateway
    let http = HttpClient::new(&sushii_conf.discord_token);

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&sushii_conf.database_url)
        .await?;

    let scheme = ShardScheme::Auto;

    let config = ClusterConfig::builder(&sushii_conf.discord_token)
        .shard_scheme(scheme)
        .intents(Some(
            GatewayIntents::GUILDS
                | GatewayIntents::GUILD_MEMBERS
                | GatewayIntents::GUILD_BANS
                | GatewayIntents::GUILD_PRESENCES
                | GatewayIntents::GUILD_MESSAGES
                | GatewayIntents::GUILD_MESSAGE_REACTIONS
                | GatewayIntents::DIRECT_MESSAGES
                | GatewayIntents::DIRECT_MESSAGE_REACTIONS,
        ))
        .build();

    // Start up the cluster
    let cluster = Cluster::new(config).await?;

    let cluster_spawn = cluster.clone();

    tokio::spawn(async move {
        cluster_spawn.up().await;
    });

    let cache_config = InMemoryConfigBuilder::new()
        .event_types(
            EventType::MESSAGE_CREATE
                | EventType::MESSAGE_DELETE
                | EventType::MESSAGE_DELETE_BULK
                | EventType::MESSAGE_UPDATE,
        )
        .build();
    let cache = InMemoryCache::from(cache_config);

    let ctx = Arc::new(model::context::SushiiContext {
        config: Arc::new(sushii_conf),
        sushii_cache: model::sushii_cache::SushiiCache::default(),
        cache: cache.clone(),
        cluster: cluster.clone(),
        http: http.clone(),
        pool: pool.clone(),
        commands: handlers::commands::create_commands(),
        // commands: Arc::new(handlers::commands::get_commands()),
    });

    let mut events = cluster.events().await;
    // Startup an event loop for each event in the event stream
    while let Some(event) = events.next().await {
        if let Err(e) = cache.update(&event.1).await {
            tracing::error!("Failed to cache event: {}", e);
        }

        let ctx = ctx.clone();
        // Spawn a new task to handle the event
        tokio::spawn(handle_event(event, ctx));
    }

    Ok(())
}

async fn handle_event<'a>(
    event: (u64, Event),
    ctx: Arc<model::context::SushiiContext<'a>>,
) -> Result<()> {
    handlers::middleware::handle_event(event.0, &event.1, ctx.clone()).await?;
    handlers::commands::handle_event(event.0, &event.1, ctx.clone()).await?;

    match event {
        (id, Event::ShardConnected(_)) => {
            println!("ShardConnected: {}", id);
        }
        (id, Event::ShardConnecting(_)) => {
            println!("ShardConnecting: {}", id);
        }
        (id, Event::ShardDisconnected(_)) => {
            println!("ShardDisconnected: {}", id);
        }
        (id, Event::ShardIdentifying(_)) => {
            println!("ShardIdentifying: {}", id);
        }
        (id, Event::ShardReconnecting(_)) => {
            println!("ShardReconnecting: {}", id);
        }
        (id, Event::ShardPayload(_)) => {
            println!("ShardPayload: {}", id);
        }
        (id, Event::ShardResuming(_)) => {
            println!("ShardResuming: {}", id);
        }
        _ => {}
    }

    Ok(())
}
