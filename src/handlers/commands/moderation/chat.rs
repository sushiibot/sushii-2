use async_trait::async_trait;
use std::sync::Arc;
use twilight::model::channel::message::Message;
use twilight::model::id::MessageId;
use twilight::command_parser::Arguments;

use crate::error::{Error, Result};
use crate::model::context::SushiiContext;
use crate::handlers::commands::CommandExec;

#[derive(Default)]
pub struct Prune;

#[async_trait]
impl CommandExec for Prune {
    async fn execute<'a>(&self, msg: &Message, ctx: Arc<SushiiContext<'a>>, args: &Arguments<'a>) -> Result<()> {
        let num_messages = args
            .as_str()
            .parse::<u64>()
            .map_err(|_| Error::UserError("Invalid input, please give a number".into()))?;
        
        if num_messages < 2 || num_messages > 100 {
            return Err(Error::UserError("Number of messages must be between 2 and 100 (inclusive)"));
        }

        // Should try to use cached messages if possible but twilight cache messages
        // aren't public and likely won't want to cache up to 100 messages per channel anyways
        let messages: Vec<MessageId> = ctx.http
            .channel_messages(msg.channel_id)
            .limit(num_messages)?
            .await?
            .iter()
            .map(|m| m.id)
            .collect();

        ctx.http
            .delete_messages(msg.channel_id, messages)
            .await?;

        Ok(())
    }
}
