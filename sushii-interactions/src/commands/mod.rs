use async_trait::async_trait;
use twilight_model::application::interaction::ApplicationCommand;

use crate::error::Result;

use self::context::CommandContext;

pub mod context;
pub mod handler;
pub mod help;
pub mod register;

#[async_trait]
pub trait ExecuteApplicationCommand {
    async fn execute_cmd(&self, ctx: CommandContext<'_>, cmd: ApplicationCommand) -> Result<()>;
}
