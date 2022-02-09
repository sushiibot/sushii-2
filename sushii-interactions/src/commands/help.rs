use async_trait::async_trait;
use twilight_interactions::command::{CommandModel, CreateCommand};
use twilight_model::application::{
    callback::{CallbackData, InteractionResponse},
    interaction::ApplicationCommand,
};

use super::{context::CommandContext, ExecuteApplicationCommand};
use crate::error::Result;

#[derive(CommandModel, CreateCommand)]
#[command(name = "help", desc = "Get help about sushii bot")]
pub struct HelpCommand {}

#[async_trait]
impl ExecuteApplicationCommand for HelpCommand {
    async fn execute_cmd(&self, ctx: CommandContext<'_>, cmd: ApplicationCommand) -> Result<()> {
        let interaction_client = ctx.http.interaction(cmd.application_id);

        interaction_client
            .interaction_callback(
                cmd.id,
                &cmd.token,
                &InteractionResponse::ChannelMessageWithSource(CallbackData {
                    allowed_mentions: None,
                    components: None,
                    content: Some("you need help lol.".into()),
                    embeds: None,
                    flags: None,
                    tts: None,
                }),
            )
            .exec()
            .await?;

        Ok(())
    }
}
