use crate::model::sushii_config::SushiiConfig;
use serenity::prelude::*;

pub async fn get(ctx: &Context) -> SushiiConfig {
    let data = ctx.data.read().await;

    data.get::<SushiiConfig>().unwrap().clone()
}
