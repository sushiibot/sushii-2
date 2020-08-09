use std::sync::Arc;
use twilight::gateway::Event;
use twilight::model::gateway::payload::MessageCreate;
use twilight::model::guild::Permissions as Perm;
use twilight::command_parser::Command;

use crate::error::{Error, Result};
use crate::model::{
    command_info::CommandInfoBuilder,
    commands::{Commands, CommandsBuilder},
    permissions::Permissions
};
use crate::model::context::SushiiContext;
use crate::utils::guards;

mod admin;
mod text;
mod user;
mod moderation;

pub fn create_commands<'a>() -> Commands<'a> {
    let cmds = CommandsBuilder::new()
        .add_command(CommandInfoBuilder::new("ping")
            .description("pong")
            .build()
        )
        .add_command(CommandInfoBuilder::new("avatar")
            .guild_only(true)
            .help("Gets avatar of a user, default yourself")
            .usage("(user)")
            .build()
        )
        .add_command(CommandInfoBuilder::new("shutdown")
            .owners_only(true)
            .build()
        )
        .add_command(CommandInfoBuilder::new("prune")
            .required_permissions(Permissions::from_permission(Perm::BAN_MEMBERS)
                .build()
            ).build()
        )
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

// Only returns Error if it isn't a user showing error: system errors
async fn exec_command<'a>(
    cmd: &Command<'a>,
    msg: &Box<MessageCreate>,
    ctx: Arc<SushiiContext<'a>>,
) -> Result<()> {
    match cmd.name {
        "ping" => text::ping(msg, ctx).await?,
        "avatar" => user::avatar(msg, ctx).await?,
        // crate::utils::macros::command!("shutdown", admin::shutdown),
        "shutdown" => admin::shutdown(msg, ctx).await?,
        "prune" => moderation::chat::prune(msg, ctx, &cmd.arguments).await?,

        _ => {}
    }

    Ok(())
}

pub async fn handle_command<'a>(
    msg: &Box<MessageCreate>,
    ctx: Arc<SushiiContext<'a>>,
) -> Result<()> {
    let prefix = &ctx.config.default_prefix;

    if msg.content.len() <= prefix.len() {
        return Ok(());
    }

    if !msg.content.starts_with(prefix) {
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
        // Want to drop the ctx borrow after checking guards so that command takes ownership
        let cmd_meta = match ctx.commands.command_list.get(cmd.name) {
            Some(m) => m,
            None => {
                tracing::warn!("Failed to get command meta info: {}", cmd.name);
                return Ok(());
            }
        };

        tracing::info!("Found command meta info: {:#?}", *cmd_meta);

        // Check for guards: permissions, guild, owner, etc
        if !guards::does_pass(msg, &cmd_meta, &ctx).await {
            return Ok(());
        }
    }

    if let Err(e) = exec_command(&cmd, msg, ctx.clone()).await {
        match e {
            Error::UserError(e) => {
                ctx.http
                    .create_message(msg.channel_id)
                    .content(e)?
                    .await?;
            }
            _ => tracing::error!(?msg, cmd = cmd.name, "Failed to run command: {}", e),
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
            if msg.author.id.0 != 150443906511667200 {
                return Ok(());
            }

            if let Err(e) = handle_command(msg, ctx.clone()).await {
                tracing::error!(?msg, "Failed to handle command: {}", e);
            };
        }
        _ => {}
    }

    Ok(())
}
