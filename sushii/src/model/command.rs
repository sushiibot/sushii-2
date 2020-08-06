use futures::future::BoxFuture;
use std::boxed::Box;
use std::future::Future;
use std::ops::Fn;
use std::pin::Pin;
use std::sync::Arc;

use twilight::gateway::Event;
use twilight::model::channel::message::Message;
use twilight::model::gateway::payload::MessageCreate;

use crate::error::Error;
use crate::model::context::SushiiContext;

// Return signature needs to be boxed since return value is unique for futures
// https://users.rust-lang.org/t/storing-async-functions-in-a-hashmap/38353/3
// pub type CommandResult = Pin<Box<dyn Future<Output = Result<()>> + Send>>;
// pub type CommandHandler = Box<dyn Fn(&Message, Arc<SushiiContext>) -> CommandResult + Send + Sync>;

pub type CommandError = Box<dyn std::error::Error + Send + Sync>;
pub type CommandResult = Result<(), CommandError>;
pub type CommandFn = fn(Arc<Message>, Arc<SushiiContext>) -> Box <dyn Future<Output=CommandResult> + 'static>;

pub struct Command {
    pub name: String,
    pub description: Option<String>,
    pub help: Option<String>,
    pub usage: Option<String>,
    // pub handler: Option<CommandHandler>,
    pub fun: Box<CommandFn>,
}

pub struct CommandBuilder(Command);

impl CommandBuilder {
    pub fn new(name: impl Into<String>, fun: Box<CommandFn>) -> Self {
        CommandBuilder(Command {
            name: name.into(),
            description: None,
            help: None,
            usage: None,
            fun,
        })
    }

    pub fn build(self) -> Command {
        self.0
    }
}
