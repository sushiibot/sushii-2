use serenity::framework::standard::macros::group;

mod list;
mod mod_log;
mod mute_role;
mod default;

use self::{list::*, mod_log::*, mute_role::*, default::*};

#[group]
#[commands(list, modlog, muterole)]
#[description("Guild settings, requires MANAGE_GUILD permissions")]
#[prefix("settings")]
#[only_in("guild")]
#[default_command(default)]
#[required_permissions("MANAGE_GUILD")]
pub struct Settings;
