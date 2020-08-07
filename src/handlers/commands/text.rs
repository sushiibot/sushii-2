use std::sync::Arc;
use std::pin::Pin;
use twilight::model::channel::message::Message;

use crate::error::Result;
use crate::model::{command::Command, context::SushiiContext};

pub async fn ping<'a>(msg: &Message, ctx: Arc<SushiiContext<'a>>) -> Result<()> {
    ctx.http
        .create_message(msg.channel_id)
        .content("pong!")?
        .await?;

    Ok(())
}
