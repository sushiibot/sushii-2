use serenity::{
    client::bridge::gateway::GatewayIntents, client::ClientBuilder, framework::StandardFramework,
    http::HttpBuilder,
};
use sqlx::postgres::PgPoolOptions;
use std::collections::HashSet;
use std::sync::Arc;
use tokio::signal::unix::{signal, SignalKind};
use tracing_subscriber::EnvFilter;

#[macro_use]
mod utils;
mod commands;
mod error;
mod handlers;
mod hooks;
mod keys;
mod model;
mod prelude;
mod tasks;

use crate::error::Result;
use crate::keys::{RedisPoolContainer, ReqwestContainer, ShardManagerContainer};
use crate::model::{sql::GuildConfig, Metrics, SushiiConfig};
use sushii_model::keys::{DbPool, SushiiCache};

#[tokio::main]
async fn main() -> Result<()> {
    let sushii_conf = Arc::new(SushiiConfig::from_env()?);

    // install global subscriber configured based on RUST_LOG envvar.
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .init();

    let _guard = sushii_conf
        .sentry_dsn
        .clone()
        .map(|url| {
            sentry::init((
                url,
                sentry::ClientOptions {
                    release: sentry::release_name!(),
                    ..Default::default()
                },
            ))
        })
        .or_else(|| {
            tracing::warn!("SENTRY_DSN is not set");

            None
        });

    let pool = PgPoolOptions::new()
        .max_connections(32)
        .connect(&sushii_conf.database_url)
        .await?;

    /*
    // Disabled for now
    let redis_pool = sushii_conf
        .redis
        .create_pool()
        .expect("Failed to create redis pool");
    */

    // sqlx::migrate!("./migrations").run(&pool).await?;

    let metrics = Arc::new(Metrics::new(&sushii_conf).await);

    let http = HttpBuilder::new(&sushii_conf.discord_token)
        .proxy(
            std::env::var("TWILIGHT_API_PROXY_URL")
                .expect("TWILIGHT_API_PROXY_URL not in environment"),
        )?
        .ratelimiter_disabled(true)
        .application_id(sushii_conf.application_id)
        .await
        .expect("Error creating Http");

    let owners = match http.get_current_application_info().await {
        Ok(info) => {
            let mut owners = HashSet::new();
            owners.insert(info.owner.id);

            owners
        }
        Err(why) => {
            tracing::error!("Could not access application info: {:?}", why);
            // Well yes, but actually no
            return Ok(());
        }
    };

    let bot_id = http.get_current_user().await?.id;

    // Create the framework
    let framework = StandardFramework::new()
        .configure(|c| {
            c.owners(owners)
                .dynamic_prefix(|ctx, msg| {
                    Box::pin(async move {
                        let sushii_conf = SushiiConfig::get(&ctx).await;

                        match GuildConfig::from_msg(&ctx, &msg).await {
                            Ok(conf) => conf
                                .and_then(|c| c.prefix)
                                .or_else(|| Some(sushii_conf.default_prefix.clone())),
                            Err(e) => {
                                tracing::error!(?msg, "Failed to get guild config: {}", e);
                                None
                            }
                        }
                    })
                })
                .prefix("")
                .on_mention(Some(bot_id))
        })
        .before(hooks::before)
        .after(hooks::after)
        .on_dispatch_error(hooks::dispatch_error)
        .group(&commands::META_GROUP)
        .group(&commands::tags::TAGS_GROUP)
        .group(&commands::feeds::FEEDS_GROUP)
        .group(&commands::guild::GUILD_GROUP)
        .group(&commands::notifications::NOTIFICATIONS_GROUP)
        .group(&commands::reminders::REMINDERS_GROUP)
        .group(&commands::lastfm::LASTFM_GROUP)
        .group(&commands::users::USERS_GROUP)
        .group(&commands::moderation::MODERATION_GROUP)
        .group(&commands::settings::SETTINGS_GROUP)
        .group(&commands::roles::ROLES_GROUP)
        .group(&commands::patreon::PATREON_GROUP)
        .group(&commands::OWNER_GROUP)
        .normal_message(hooks::normal_message)
        .bucket("rank", |b| b.delay(5))
        .await;

    let mut client = ClientBuilder::new_with_http(http)
        .intents(
            GatewayIntents::GUILDS
                | GatewayIntents::GUILD_MEMBERS
                | GatewayIntents::GUILD_BANS
                | GatewayIntents::GUILD_MESSAGES
                | GatewayIntents::GUILD_MESSAGE_REACTIONS
                | GatewayIntents::DIRECT_MESSAGES
                | GatewayIntents::DIRECT_MESSAGE_REACTIONS,
        )
        .framework(framework)
        .event_handler(handlers::Handler)
        .raw_event_handler(handlers::RawHandler)
        .await
        .expect("Err creating client");

    // Add data to client
    {
        let mut data = client.data.write().await;
        data.insert::<ShardManagerContainer>(client.shard_manager.clone());

        data.insert::<SushiiConfig>(Arc::clone(&sushii_conf));
        data.insert::<SushiiCache>(SushiiCache::default());
        data.insert::<DbPool>(pool.clone());
        // data.insert::<RedisPoolContainer>(redis_pool.clone());
        data.insert::<Metrics>(Arc::clone(&metrics));
        data.insert::<ReqwestContainer>(reqwest::Client::new());
    }

    let signal_kinds = vec![
        SignalKind::hangup(),
        SignalKind::interrupt(),
        SignalKind::terminate(),
    ];

    for signal_kind in signal_kinds {
        let mut stream = signal(signal_kind).unwrap();
        let shard_manager = client.shard_manager.clone();
        let pool = pool.clone();

        tokio::spawn(async move {
            stream.recv().await;
            tracing::info!("Signal received, shutting down...");
            shard_manager.lock().await.shutdown_all().await;

            tracing::info!("Closing database pool...");
            pool.close().await;

            tracing::info!("Shutting down metrics server...");

            tracing::info!("bye");
        });
    }

    if let Err(why) = client.start_autosharded().await {
        tracing::error!("Client error: {:?}", why);
    }

    Ok(())
}
