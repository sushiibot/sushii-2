use futures::future::BoxFuture;
use std::boxed::Box;
use std::future::Future;
use std::ops::Fn;
use std::pin::Pin;
use std::sync::Arc;

use twilight::gateway::Event;
use twilight::model::channel::message::Message;
use twilight::model::gateway::payload::MessageCreate;
use twilight::model::guild::Permissions;

use crate::error::Error;
use crate::model::context::SushiiContext;

#[derive(Debug)]
pub struct Command {
    pub name: String,
    pub description: Option<String>,
    pub help: Option<String>,
    pub usage: Option<String>,
    pub owners_only: bool,
    pub required_permissions: Option<Permissions>,
    pub guild_only: bool
}

#[derive(Debug)]
pub struct CommandBuilder(Command);

impl CommandBuilder {
    pub fn new(name: impl Into<String>) -> Self {
        CommandBuilder(Command {
            name: name.into(),
            description: None,
            help: None,
            usage: None,
            owners_only: false,
            required_permissions: None,
            guild_only: false
        })
    }

    pub fn description(mut self, desc: impl Into<String>) -> Self {
        self.0.description.replace(desc.into());
        self
    }

    pub fn help(mut self, help: impl Into<String>) -> Self {
        self.0.help.replace(help.into());
        self
    }

    pub fn usage(mut self, usage: impl Into<String>) -> Self {
        self.0.usage.replace(usage.into());
        self
    }

    pub fn owners_only(mut self, owners_only: bool) -> Self {
        self.0.owners_only = owners_only;
        self
    }

    pub fn required_permissions(mut self, required_permissions: Permissions) -> Self {
        self.0.required_permissions.replace(required_permissions);
        self
    }

    pub fn guild_only(mut self, guild_only: bool) -> Self {
        self.0.guild_only = guild_only;
        self
    }

    pub fn build(self) -> Command {
        self.0
    }
}
