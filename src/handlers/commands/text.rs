use async_trait::async_trait;
use std::sync::Arc;
use twilight::model::channel::message::Message;
use twilight::command_parser::Arguments;

use crate::error::Result;
use crate::model::context::SushiiContext;

use crate::handlers::commands::CommandExec;

#[derive(Default)]
pub struct Ping;

#[async_trait]
impl CommandExec for Ping {
    async fn execute<'a>(&self, msg: &Message, ctx: Arc<SushiiContext<'a>>, _args: &Arguments<'a>) -> Result<()> {
        tracing::info!(?msg, "pinging");

        ctx.http
            .create_message(msg.channel_id)
            .content("pong!")?
            .await?;

        Ok(())
    }
}
