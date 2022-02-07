use twilight_interactions::command::{CommandModel, CreateCommand, ResolvedUser};

#[derive(CommandModel, CreateCommand)]
#[command(name = "help", desc = "Get help about sushii bot")]
pub struct HelpCommand {}
