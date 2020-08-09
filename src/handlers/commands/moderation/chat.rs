use std::sync::Arc;
use twilight::model::channel::message::Message;
use twilight::model::id::MessageId;
use twilight::command_parser::Arguments;

use crate::error::{Error, Result};
use crate::model::context::SushiiContext;

pub async fn prune<'a>(msg: &Message, ctx: Arc<SushiiContext<'a>>, args: &Arguments<'a>) -> Result<()> {
    let num_messages = args
        .as_str()
        .parse::<u64>()
        .map_err(|_| Error::UserError("Invalid input, please give a number".into()))?;

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
