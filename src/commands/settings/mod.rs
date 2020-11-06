use serenity::framework::standard::macros::group;

mod default;
mod list;
mod mute;

use self::{default::*, list::*, mute::*};

#[group]
#[commands(list, mute)]
#[description("Guild settings, requires MANAGE_GUILD permissions")]
#[prefix("settings")]
#[only_in("guild")]
#[default_command(default)]
#[required_permissions("MANAGE_GUILD")]
pub struct Settings;
