use twilight::model::guild::Permissions;
use crate::handlers::commands::CommandExec;

#[derive(Debug)]
pub struct CommandInfo {
    pub name: String,
    pub description: Option<String>,
    pub help: Option<String>,
    pub usage: Option<String>,
    pub owners_only: bool,
    pub required_permissions: Permissions,
    pub guild_only: bool,
    pub exec: Option<Box<dyn CommandExec + Send>>,
}

#[derive(Debug)]
pub struct CommandInfoBuilder(CommandInfo);

impl CommandInfoBuilder {
    pub fn new(name: impl Into<String>) -> Self {
        CommandInfoBuilder(CommandInfo {
            name: name.into(),
            description: None,
            help: None,
            usage: None,
            owners_only: false,
            required_permissions: Permissions::empty(),
            guild_only: false,
            exec: None,
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
        self.0.required_permissions = required_permissions;
        self
    }

    pub fn guild_only(mut self, guild_only: bool) -> Self {
        self.0.guild_only = guild_only;
        self
    }

    pub fn exec(mut self, exec: Box<dyn CommandExec + Send>) -> Self {
        self.0.exec.replace(exec);
        self
    }

    pub fn build(self) -> CommandInfo {
        if self.0.exec.is_none() {
            tracing::error!("Missing exec function for command: {}", self.0.name);
        }

        self.0
    }
}
