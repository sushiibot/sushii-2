use sqlx::postgres::PgPoolOptions;

use serenity::{
    framework::StandardFramework, http::Http, prelude::*,
};
use std::{collections::HashSet, sync::Arc};

mod commands;
mod error;
mod handler;
mod hooks;
// mod handlers;
mod model;
// mod utils;

use crate::error::Result;
use crate::model::{
    context::SushiiContext, shardmanager::ShardManagerContainer, sushii_cache::SushiiCache,
};

#[tokio::main]
async fn main() -> Result<()> {
    // install global subscriber configured based on RUST_LOG envvar.
    tracing_subscriber::fmt().init();

    let sushii_conf =
        model::sushii_config::SushiiConfig::new_from_env().expect("failed to make config");

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&sushii_conf.database_url)
        .await?;

    let sushii_ctx = SushiiContext {
        config: Arc::new(sushii_conf),
        sushii_cache: SushiiCache::default(),
        pool: pool.clone(),
    };

    let http = Http::new_with_token(&sushii_ctx.config.discord_token);

    // We will fetch your bot's owners and id
    let (owners, _bot_id) = match http.get_current_application_info().await {
        Ok(info) => {
            let mut owners = HashSet::new();
            owners.insert(info.owner.id);

            (owners, info.id)
        }
        Err(why) => panic!("Could not access application info: {:?}", why),
    };

    // Create the framework
    let framework = StandardFramework::new()
        .configure(|c| {
            c.owners(owners).prefix("s!").dynamic_prefix(|ctx, msg| {
                Box::pin(async move {
                    let data = ctx.data.read().await;
                    let sushii_ctx = data.get::<SushiiContext>().unwrap();

                    msg.guild_id
                        .and_then(|g| sushii_ctx.sushii_cache.guilds.get(&g))
                        .and_then(|c| c.prefix.clone())
                })
            })
        })
        .before(hooks::before)
        .group(&commands::META_GROUP)
        .group(&commands::moderation::MODERATION_GROUP)
        .group(&commands::OWNER_GROUP);

    let mut client = Client::new(&sushii_ctx.config.discord_token)
        .framework(framework)
        .event_handler(handler::Handler)
        .await
        .expect("Err creating client");

    // Add data to client
    {
        let mut data = client.data.write().await;
        data.insert::<SushiiContext>(sushii_ctx);
        data.insert::<ShardManagerContainer>(client.shard_manager.clone());
    }

    if let Err(why) = client.start().await {
        tracing::error!("Client error: {:?}", why);
    }

    Ok(())
}

/*
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
*/
