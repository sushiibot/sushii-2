use twilight_http::Client;
use twilight_interactions::command::CommandModel;
use twilight_model::application::interaction::ApplicationCommand;

use crate::error::Result;

use super::context::CommandContext;
use super::help::HelpCommand;
use super::ExecuteApplicationCommand;

pub async fn handle_command(command: ApplicationCommand, client: &Client) -> Result<()> {
    let interaction_name = &command.data.name;

    let ctx = CommandContext { http: client };

    match interaction_name.as_str() {
        "help" => {
            let help_cmd: HelpCommand = HelpCommand::from_interaction(command.data.clone().into())?;
            help_cmd.execute_cmd(ctx, command).await
        }
        _ => Ok(()),
    }
}
