use std::sync::Arc;
use twilight::model::channel::message::Message;

use crate::model::context::SushiiContext;
use crate::model::command::Command;
use crate::error::{Error, Result};

pub fn is_bot(msg: &Message) -> bool {
    msg.author.bot
}

pub fn is_bot_owner(msg: &Message, ctx: &Arc<SushiiContext>) -> bool {
    ctx.config.owner_ids.contains(&msg.author.id.0)
}

pub fn is_guild(msg: &Message) -> bool {
    msg.guild_id.is_some()
}

pub async fn does_pass<'a>(msg: &Message, cmd_meta: &Command, ctx: &Arc<SushiiContext<'a>>) -> bool {
    match check_guards(msg, cmd_meta, ctx) {
        Ok(()) => true,
        Err(e) => {
            if let Err(e) = respond_err(msg, ctx, e).await {
                tracing::warn!("Failed to send guard fail message: {}", e);
            }

            false
        }
    }
}

pub async fn respond_err<'a>(msg: &Message, ctx: &Arc<SushiiContext<'a>>, err: Error) -> Result<()> {
    ctx.http
        .create_message(msg.channel_id)
        .content(format!("{}", err))?
        .await?;
    
    Ok(())
}

pub fn check_guards(msg: &Message, cmd_meta: &Command, ctx: &Arc<SushiiContext>) -> Result<()> {
    if is_bot(msg) {
        return Err(Error::IsBot);
    }

    if cmd_meta.owners_only && !is_bot_owner(msg, ctx) {
        return Err(Error::Sushii("no, sorry".into()));
    }

    if cmd_meta.guild_only && !is_guild(msg) {
        return Err(Error::Sushii("You can only use this command in guilds".into()));
    }

    Ok(())
}