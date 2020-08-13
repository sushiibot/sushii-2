use async_trait::async_trait;
use std::sync::Arc;
use twilight::builders::embed::EmbedBuilder;
use twilight::model::channel::message::Message;
use twilight::command_parser::Arguments;

use crate::error::Result;
use crate::model::context::SushiiContext;

use crate::handlers::commands::CommandExec;

#[derive(Default)]
pub struct Avatar;

#[async_trait]
impl CommandExec for Avatar {
    async fn execute<'a>(&self, msg: &Message, ctx: Arc<SushiiContext<'a>>, _args: &Arguments<'a>) -> Result<()> {
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
}
