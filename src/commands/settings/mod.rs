use serenity::framework::standard::macros::group;

mod default;
mod list;
mod mod_log;
mod mute_role;

use self::{default::*, list::*, mod_log::*, mute_role::*};

#[group]
#[commands(list, modlog, muterole)]
#[description("Guild settings, requires MANAGE_GUILD permissions")]
#[prefix("settings")]
#[only_in("guild")]
#[default_command(default)]
#[required_permissions("MANAGE_GUILD")]
pub struct Settings;
