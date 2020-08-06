use std::sync::Arc;
use twilight::model::channel::message::Message;

use crate::error::Result;
use crate::model::context::SushiiContext;
use crate::utils::guards;

pub async fn shutdown<'a>(msg: &Message, ctx: Arc<SushiiContext<'a>>) -> Result<()> {
    guards::is_bot_owner(msg, &ctx);

    ctx.http
        .create_message(msg.channel_id)
        .content("bye")?
        .await?;

    ctx.cluster.down().await;

    Ok(())
}
