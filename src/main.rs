use serenity::{
    client::bridge::gateway::GatewayIntents, framework::StandardFramework, http::Http, prelude::*,
};
use sqlx::postgres::PgPoolOptions;
use std::collections::HashSet;

mod commands;
mod error;
mod handlers;
mod hooks;
mod keys;
mod model;
mod prelude;
mod utils;

use crate::error::Result;
use crate::keys::{DbPool, ShardManagerContainer};
use crate::model::{
    sql::{GuildConfig, GuildConfigDb},
    SushiiCache, {SushiiConfig, SushiiConfigDb},
};

#[tokio::main]
async fn main() -> Result<()> {
    // install global subscriber configured based on RUST_LOG envvar.
    tracing_subscriber::fmt().init();

    let sushii_conf = SushiiConfig::new_from_env().expect("failed to make config");

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&sushii_conf.database_url)
        .await?;

    let http = Http::new_with_token(&sushii_conf.discord_token);

    let (owners, _bot_id) = match http.get_current_application_info().await {
        Ok(info) => {
            let mut owners = HashSet::new();
            owners.insert(info.owner.id);

            (owners, info.id)
        }
        Err(why) => {
            tracing::error!("Could not access application info: {:?}", why);
            // Well yes, but actually no
            return Ok(());
        }
    };

    // Create the framework
    let framework = StandardFramework::new()
        .configure(|c| {
            c.owners(owners).dynamic_prefix(|ctx, msg| {
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
        })
        .before(hooks::before)
        .after(hooks::after)
        .on_dispatch_error(hooks::dispatch_error)
        .help(&commands::help::HELP_CMD)
        .group(&commands::META_GROUP)
        .group(&commands::moderation::MODERATION_GROUP)
        .group(&commands::roles::ROLES_GROUP)
        .group(&commands::OWNER_GROUP);

    let mut client = Client::new(&sushii_conf.discord_token)
        .intents(
            GatewayIntents::GUILDS
                | GatewayIntents::GUILD_MEMBERS
                | GatewayIntents::GUILD_BANS
                | GatewayIntents::GUILD_MESSAGES
                | GatewayIntents::GUILD_MESSAGE_REACTIONS
                | GatewayIntents::DIRECT_MESSAGES
                | GatewayIntents::GUILD_PRESENCES,
        )
        .framework(framework)
        .event_handler(handlers::Handler)
        .await
        .expect("Err creating client");

    // Add data to client
    {
        let mut data = client.data.write().await;
        data.insert::<ShardManagerContainer>(client.shard_manager.clone());

        data.insert::<SushiiConfig>(sushii_conf);
        data.insert::<SushiiCache>(SushiiCache::default());
        data.insert::<DbPool>(pool);
    }

    if let Err(why) = client.start().await {
        tracing::error!("Client error: {:?}", why);
    }

    Ok(())
}
