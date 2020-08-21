use serenity::framework::standard::{macros::command, Args, CommandResult};
use serenity::model::prelude::*;
use serenity::prelude::*;

use crate::keys::{DbPool, ShardManagerContainer};

#[command]
#[owners_only]
async fn quit(ctx: &Context, msg: &Message) -> CommandResult {
    let data = ctx.data.read().await;

    let manager = data.get::<ShardManagerContainer>().unwrap();
    msg.channel_id.say(ctx, "bye").await?;

    manager.lock().await.shutdown_all().await;

    Ok(())
}

/*
#[command]
#[owners_only]
async fn query(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    let query_str = args.rest();

    if query_str.is_empty() {
        msg.channel_id
            .say(&ctx.http, "Error: No query given")
            .await?;
    }

    let data = ctx.data.read().await;
    let pool = data.get::<DbPool>().unwrap();

    // How to return a generic value like serde_json::Value? Doesn't seem like sqlx supports yet
    // https://github.com/launchbadge/sqlx/issues/182
    let res = sqlx::query(query_str).fetch_all(pool).await;

    match res {
        Ok(rows) => {
            msg.channel_id
                .say(&ctx.http, format!("```{:#?}```", rows))
                .await?;
        }
        Err(e) => {
            msg.channel_id
                .say(&ctx.http, format!("Error: `{}`", e))
                .await?;
        }
    }

    Ok(())
}
*/
