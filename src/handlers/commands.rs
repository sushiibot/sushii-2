use std::sync::Arc;
use std::fmt::Debug;
use twilight::gateway::Event;
use twilight::model::channel::message::Message;
use twilight::model::gateway::payload::MessageCreate;
use twilight::model::guild::Permissions as Perm;
use twilight::command_parser::Arguments;

use crate::error::{Error, Result};
use crate::model::{
    command_info::CommandInfoBuilder,
    commands::{Commands, CommandsBuilder},
    permissions::Permissions
};
use crate::model::context::SushiiContext;
use crate::utils::guards;

use async_trait::async_trait;

mod owner;
mod text;
mod user;
mod moderation;

#[async_trait]
pub trait CommandExec: Sync {
    // Default ::new() just returns a boxed of Self
    fn new() -> Box<Self> where Self: Sized + Default {
        Box::new(Self::default())
    }

    async fn execute<'a>(&self, msg: &Message, ctx: Arc<SushiiContext<'a>>, args: &Arguments<'a>) -> Result<()>;
}

impl Debug for (dyn CommandExec + std::marker::Send + 'static) {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "CommandExec")
    }
}

pub fn create_commands<'a>() -> Commands<'a> {
    let cmds = CommandsBuilder::new()
        .add_command(CommandInfoBuilder::new("ping")
            .description("pong")
            .exec(text::Ping::new())
            .build()
        )
        .add_command(CommandInfoBuilder::new("avatar")
            .exec(user::Avatar::new())
            .guild_only(true)
            .help("Gets avatar of a user, default yourself")
            .usage("(user)")
            .build()
        )
        .add_command(CommandInfoBuilder::new("shutdown")
            .exec(owner::Shutdown::new())
            .owners_only(true)
            .build()
        )
        .add_command(CommandInfoBuilder::new("prune")
            .exec(moderation::chat::Prune::new())
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
/*
async fn exec_command<'a>(
    cmd: &Command<'a>,
    msg: &Box<MessageCreate>,
    ctx: Arc<SushiiContext<'a>>,
) -> Result<()> {
    match cmd.name {
        "ping" => text::ping(msg, ctx).await?,
        "avatar" => user::avatar(msg, ctx).await?,
        // crate::utils::macros::command!("shutdown", admin::shutdown),
        "prune" => moderation::chat::prune(msg, ctx, &cmd.arguments).await?,

        // Owner commands
        "shutdown" => owner::shutdown(msg, ctx).await?,

        _ => {
            tracing::error!("Command parsed but isn't executed: {}", cmd.name);
        }
    }

    Ok(())
}
*/

pub async fn handle_command<'a>(
    msg: &Box<MessageCreate>,
    ctx: Arc<SushiiContext<'a>>,
) -> Result<()> {
    let guild_conf = msg.guild_id.and_then(|g| ctx.sushii_cache.guilds.get(&g));

    // If guild && guild.prefix, use guild prefix, else use default_prefix
    let prefix = &guild_conf.and_then(|c| c.prefix.clone()).unwrap_or_else(|| ctx.config.default_prefix.clone());

    if msg.content.len() <= prefix.len() {
        return Ok(());
    }

    if !msg.content.starts_with(&prefix[..]) {
        return Ok(());
    }

    let full_command = &msg.content[prefix.len()..];

    tracing::info!("Found command: {}", full_command);

    // Parse command
    let cmd_match = match ctx.commands.parser.parse(full_command) {
        Some(c) => c,
        None => {
            tracing::info!("No parse match found: {}", full_command);
            return Ok(())
        }
    };


    // Get command meta info (permissions, info, help, etc)
    // Want to drop the ctx borrow after checking guards so that command takes ownership
    let cmd = match ctx.commands.command_list.get(cmd_match.name) {
        Some(m) => m,
        None => {
            tracing::warn!("Failed to get command meta info: {}", cmd_match.name);
            return Ok(());
        }
    };

    tracing::info!("Found command meta info: {:#?}", *cmd);

    // Check for guards: permissions, guild, owner, etc
    if !guards::does_pass(msg, &cmd, &ctx).await {
        return Ok(());
    }

    let exec = match &cmd.exec {
        Some(e) => e,
        None => return Err(Error::Sushii(format!("Missing command exec function: {}", &cmd.name))),
    };

    if let Err(e) = exec.execute(msg, ctx.clone(), &cmd_match.arguments).await {
        match e {
            Error::UserError(e) => {
                ctx.http
                    .create_message(msg.channel_id)
                    .content(e)?
                    .await?;
            }
            _ => return Err(e.into()),
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
