use super::help::HelpCommand;
use twilight_http::Client;
use twilight_interactions::command::CreateCommand;
use twilight_model::id::marker::{ApplicationMarker, GuildMarker};
use twilight_model::id::Id;

use crate::error::Result;

pub async fn register_commands(
    client: &Client,
    application_id: Id<ApplicationMarker>,
) -> Result<()> {
    let interaction_client = client.interaction(application_id);
    let guild_id = Id::<GuildMarker>::new(167058919611564043);

    let commands = interaction_client
        .set_guild_commands(guild_id, &[HelpCommand::create_command().into()])
        .exec()
        .await?
        .models()
        .await?;

    tracing::info!("Registered {} global commands", commands.len());

    Ok(())
}
