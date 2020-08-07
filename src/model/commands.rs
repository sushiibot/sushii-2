use std::sync::Arc;

use twilight::command_parser::{CommandParserConfig, Parser};
use dashmap::DashMap;

use crate::model::command::Command;

#[derive(Clone, Debug)]
pub struct Commands<'a> {
    pub parser: Parser<'a>,
    pub command_list: Arc<DashMap<String, Command>>,
}

pub struct CommandsBuilder<'a> {
    parser_config: CommandParserConfig<'a>,
    command_list: DashMap<String, Command>,
}

impl<'a> CommandsBuilder<'a> {
    pub fn new() -> Self {
        let mut parser_config = CommandParserConfig::new();
        // Add blank prefix so we can set our own dynamically later
        parser_config.add_prefix("");

        CommandsBuilder {
            parser_config,
            command_list: DashMap::new(),
        }
    }

    pub fn add_command(mut self, cmd: Command) -> Self {
        tracing::debug!("Adding command: {:#?}", cmd);
        self.parser_config.add_command(cmd.name.clone(), false);
        self.command_list.insert(cmd.name.clone(), cmd);
        self
    }

    pub fn build(self) -> Commands<'a> {
        Commands {
            parser: Parser::new(self.parser_config),
            command_list: Arc::new(self.command_list),
        }
    }
}
