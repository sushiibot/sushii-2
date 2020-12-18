use crate::keys::Metrics;
use serenity::{async_trait, model::prelude::*, prelude::*};

pub struct RawHandler;

#[async_trait]
impl RawEventHandler for RawHandler {
    async fn raw_event(&self, ctx: Context, event: Event) {
        let data = ctx.data.read().await;
        let metrics = data.get::<Metrics>().unwrap();

        metrics.raw_event(&ctx, &event).await;
    }
}
