use crate::keys::{Metrics, RedisPoolContainer};
use redis::AsyncCommands;
use serenity::{async_trait, model::prelude::*, prelude::*};

pub struct RawHandler;

#[async_trait]
impl RawEventHandler for RawHandler {
    async fn raw_event(&self, ctx: Context, event: Event) {
        let metrics = ctx.data.read().await.get::<Metrics>().cloned().unwrap();

        metrics.raw_event(&ctx, &event).await;

        /*
        let event_type = event.event_type();

        // Guild unavailable = None
        let event_name = match event_type.name() {
            Some(n) => n,
            None => return,
        };

        let event_str = match serde_json::to_string(&event) {
            Ok(s) => s,
            Err(e) => {
                tracing::warn!("Failed to convert event to string: {}", e);
                return;
            }
        };

        let payload_str = match serde_json::to_string(&serde_json::json!({
            "name": event_name,
            "payload": event_str,
        })) {
            Ok(s) => s,
            Err(e) => {
                tracing::warn!("Failed to convert payload to string: {}", e);
                return;
            }
        };

        let redis_pool = ctx
            .data
            .read()
            .await
            .get::<RedisPoolContainer>()
            .cloned()
            .unwrap();

        let mut conn = redis_pool.get().await.unwrap();

        if let Err(e) = conn
            .lpush::<&str, String, usize>("events:0", payload_str)
            .await
        {
            tracing::error!("Failed to push event to redis: {}", e);
        }
        */
    }
}
