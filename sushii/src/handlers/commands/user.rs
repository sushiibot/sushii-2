use std::sync::Arc;
use twilight::builders::embed::EmbedBuilder;
use twilight::model::channel::message::Message;

use crate::error::Result;
use crate::model::context::SushiiContext;

pub async fn avatar<'a>(msg: &Message, ctx: Arc<SushiiContext<'a>>) -> Result<()> {
    let avatar = msg.author.avatar.clone().unwrap_or_else(|| {
        format!(
            "https://cdn.discordapp.com/embed/avatars/{}.png",
            msg.author.discriminator.parse::<u16>().unwrap_or_default() % 5
        )
    });

    ctx.http
        .create_message(msg.channel_id)
        .embed(
            EmbedBuilder::new()
                .title(&msg.author.name)
                .image(avatar)
                .build(),
        )?
        .await?;

    Ok(())
}
