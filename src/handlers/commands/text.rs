use async_trait::async_trait;
use std::sync::Arc;
use twilight::model::channel::message::Message;

use crate::error::Result;
use crate::model::context::SushiiContext;

use super::CommandExec;

#[derive(Default)]
pub struct Ping;

#[async_trait]
impl CommandExec for Ping {
    async fn execute<'a>(&self, msg: &Message, ctx: Arc<SushiiContext<'a>>) -> Result<()> {
        tracing::info!(?msg, "pinging");

        ctx.http
            .create_message(msg.channel_id)
            .content("pong!")?
            .await?;

        Ok(())
    }
}
