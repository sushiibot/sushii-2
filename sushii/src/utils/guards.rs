use std::sync::Arc;
use twilight::model::channel::message::Message;

use crate::model::context::SushiiContext;

pub fn is_bot(msg: &Message) -> bool {
    msg.author.bot
}

pub fn is_bot_owner(msg: &Message, ctx: &Arc<SushiiContext>) -> bool {
    ctx.config.owner_ids.contains(&msg.author.id.0)
}
