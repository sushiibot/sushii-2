use twilight_http::Client;
use twilight_interactions::command::CommandModel;
use twilight_model::application::interaction::{ApplicationCommand, Interaction};

use crate::error::Result;

use super::context::CommandContext;
use super::help::HelpCommand;
use super::user::userinfo::UserinfoCommand;
use super::ExecuteApplicationCommand;

pub async fn handle_interaction(interaction: Interaction, client: &Client) {
    match interaction {
        Interaction::ApplicationCommand(command) => {
            if let Err(e) = handle_command(*command, client).await {
                tracing::error!("Failed to handle application command: {}", e);
            }
        }
        _ => {}
    }
}

pub async fn handle_command(command: ApplicationCommand, client: &Client) -> Result<()> {
    let interaction_name = &command.data.name;

    let ctx = CommandContext { http: client };

    match interaction_name.as_str() {
        "help" => {
            HelpCommand::from_interaction(command.data.clone().into())?
                .execute_cmd(ctx, command)
                .await
        }
        "userinfo" => {
            UserinfoCommand::from_interaction(command.data.clone().into())?
                .execute_cmd(ctx, command)
                .await
        }
        _ => Ok(()),
    }
}
