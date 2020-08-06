use std::sync::Arc;
use twilight::command_parser::{CommandParserConfig, Parser};
use twilight::gateway::Event;
use twilight::model::gateway::payload::MessageCreate;

use crate::error::Result;
use crate::model::command::{Command, CommandBuilder};
use crate::model::context::SushiiContext;
use crate::utils::guards::is_bot;

mod admin;
mod text;
mod user;

pub fn create_command_parser<'a>() -> Parser<'a> {
    let mut config = CommandParserConfig::new();

    config.add_command("echo", false);
    config.add_command("avatar", false);
    config.add_command("ping", false);
    config.add_command("shutdown", false);
    config.add_prefix("");

    Parser::new(config)
}

/*
commands map no work idk
pub fn get_commands() -> dashmap::DashMap<String, Command> {
    let map = dashmap::DashMap::new();
    map.insert(
        "ping".into(),
        CommandBuilder::new("ping", Box::new(|msg, ctx| Box::pin(text::ping(msg, ctx)))).build(),
    );

    map
}
*/

pub async fn handle_command<'a>(
    msg: &Box<MessageCreate>,
    ctx: Arc<SushiiContext<'a>>,
) -> Result<()> {
    let prefix = "s!";

    if msg.content.len() <= prefix.len() {
        return Ok(());
    }

    if !msg.content.starts_with(&prefix) {
        return Ok(());
    }

    let full_command = &msg.content[prefix.len()..];

    if let Some(cmd) = ctx.command_parser.parse(full_command) {
        match cmd.name {
            // "ping" => text::ping(msg, ctx).await?,
            "avatar" => user::avatar(msg, ctx).await?,
            // crate::utils::macros::command!("shutdown", admin::shutdown),
            "shutdown" => admin::shutdown(msg, ctx).await?,

            _ => {}
        }
    }

    Ok(())
}

pub async fn handle_event<'a>(
    _shard_id: u64,
    event: &Event,
    ctx: Arc<SushiiContext<'a>>,
) -> Result<()> {
    match event {
        Event::MessageCreate(msg) => {
            if is_bot(msg) {
                return Ok(());
            }

            if msg.author.id.0 != 150443906511667200 {
                return Ok(());
            }

            if let Err(e) = handle_command(msg, ctx).await {
                println!("Failed to run command: {}", e);
            };
        }
        _ => {}
    }

    Ok(())
}
