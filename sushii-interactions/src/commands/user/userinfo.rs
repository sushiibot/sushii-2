use async_trait::async_trait;
use twilight_embed_builder::{EmbedBuilder, EmbedFieldBuilder, ImageSource};
use twilight_interactions::command::{CommandModel, CreateCommand};
use twilight_model::application::{
    callback::{CallbackData, InteractionResponse},
    interaction::ApplicationCommand,
};
use twilight_model::channel::message::MessageFlags;
use twilight_model::user::PremiumType;
use twilight_model::user::User;
use twilight_util::builder::CallbackDataBuilder;

use crate::cdn::{Extension, ImageSize, UserImage};
use crate::commands::{context::CommandContext, ExecuteApplicationCommand};
use crate::error::Result;

#[derive(CommandModel, CreateCommand)]
#[command(name = "userinfo", desc = "Get information about a user")]
pub struct UserinfoCommand {
    /// The user to get information about.
    user: Option<User>,
}

#[async_trait]
impl ExecuteApplicationCommand for UserinfoCommand {
    async fn execute_cmd(&self, ctx: CommandContext<'_>, cmd: ApplicationCommand) -> Result<()> {
        let interaction_client = ctx.http.interaction(cmd.application_id);

        // If user provided, then member.user, then cmd.user if in DMs
        let target = match self
            .user
            .as_ref()
            .or_else(|| cmd.member.as_ref().and_then(|m| m.user.as_ref()))
            .or_else(|| cmd.user.as_ref())
        {
            Some(u) => u,
            None => {
                let callback_data = CallbackDataBuilder::new()
                    .content("Uh oh, failed to find user.".to_string())
                    .build();

                interaction_client
                    .interaction_callback(
                        cmd.id,
                        &cmd.token,
                        &InteractionResponse::ChannelMessageWithSource(callback_data),
                    )
                    .exec()
                    .await?;

                return Ok(());
            }
        };

        let mut embed_builder = EmbedBuilder::new()
            .title("User info")
            .field(EmbedFieldBuilder::new("Username", target.name.clone()))
            .thumbnail(ImageSource::url(
                target
                    .avatar(Extension::PNG, false, ImageSize::Large)
                    .to_string(),
            )?);

        match target.premium_type {
            Some(PremiumType::NitroClassic) => {
                embed_builder =
                    embed_builder.field(EmbedFieldBuilder::new("Nitro", "Nitro Classic"));
            }
            Some(PremiumType::Nitro) => {
                embed_builder =
                    embed_builder.field(EmbedFieldBuilder::new("Nitro", "Nitro with Boost"));
            }
            _ => {}
        };

        interaction_client
            .interaction_callback(
                cmd.id,
                &cmd.token,
                &InteractionResponse::ChannelMessageWithSource(CallbackData {
                    allowed_mentions: None,
                    components: None,
                    content: None,
                    embeds: Some(vec![embed_builder.build()?]),
                    flags: Some(MessageFlags::EPHEMERAL),
                    tts: None,
                }),
            )
            .exec()
            .await?;

        Ok(())
    }
}
