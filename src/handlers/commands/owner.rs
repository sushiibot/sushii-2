use async_trait::async_trait;
use std::sync::Arc;
use twilight::model::channel::message::Message;
use twilight::command_parser::Arguments;

use crate::error::Result;
use crate::model::context::SushiiContext;
use crate::handlers::commands::CommandExec;

#[derive(Default)]
pub struct Shutdown;

#[async_trait]
impl CommandExec for Shutdown {
    async fn execute<'a>(&self, msg: &Message, ctx: Arc<SushiiContext<'a>>, _args: &Arguments<'a>) -> Result<()> {
        ctx.http
            .create_message(msg.channel_id)
            .content("bye")?
            .await?;
    
        ctx.cluster.down().await;
    
        Ok(())
    }
}
