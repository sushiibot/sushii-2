use std::sync::Arc;
use twilight::command_parser::{CommandParserConfig, Parser};
use twilight::gateway::Event;
use twilight::model::gateway::payload::MessageCreate;

use crate::error::Result;
use crate::model::{command::{Command, CommandBuilder}, commands::{Commands, CommandsBuilder}};
use crate::model::context::SushiiContext;
use crate::utils::guards;

mod admin;
mod text;
mod user;

pub fn create_commands<'a>() -> Commands<'a> {
    let cmds = CommandsBuilder::new()
        .add_command(CommandBuilder::new("ping").build())
        .add_command(CommandBuilder::new("avatar").guild_only(true).build())
        .add_command(CommandBuilder::new("shutdown").owners_only(true).build())
        .build();

    tracing::info!("Commands added: {:#?}", cmds.parser.config().commands());

    cmds
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

    tracing::info!("Found command: {}", full_command);


    // Parse command
    let cmd = match ctx.commands.parser.parse(full_command) {
        Some(c) => c,
        None => {
            tracing::info!("No parse match found: {}", full_command);
            return Ok(())
        }
    };

    {
        // Get command meta info (permissions, info, help, etc)
        let cmd_meta = match ctx.commands.command_list.get(cmd.name) {
            Some(m) => m,
            None => {
                tracing::warn!("Failed to get command meta info: {}", cmd.name);
                return Ok(());
            }
        };
    
        tracing::info!("Found command meta info: {:#?}", *cmd_meta);

        if !guards::does_pass(msg, &cmd_meta, &ctx).await {
            return Ok(());
        }
    }

    match cmd.name {
        "ping" => text::ping(msg, ctx).await?,
        "avatar" => user::avatar(msg, ctx).await?,
        // crate::utils::macros::command!("shutdown", admin::shutdown),
        "shutdown" => admin::shutdown(msg, ctx).await?,

        _ => {}
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
