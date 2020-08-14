use serenity::framework::standard::macros::hook;
use serenity::model::prelude::*;
use serenity::prelude::*;

mod cacher;

#[hook]
pub async fn before(ctx: &Context, msg: &Message, cmd_name: &str) -> bool {
    tracing::info!("Running command {}", cmd_name);

    cacher::cache_guild_config(ctx, msg).await;

    true
}
