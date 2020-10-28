use serenity::framework::standard::macros::group;

mod default;
mod join_msg;
mod leave_msg;
mod list;
mod mod_log;
mod mute;

use self::{default::*, join_msg::*, leave_msg::*, list::*, mod_log::*, mute::*};

#[group]
#[commands(list, modlog, mute, joinmsg, leavemsg)]
#[description("Guild settings, requires MANAGE_GUILD permissions")]
#[prefix("settings")]
#[only_in("guild")]
#[default_command(default)]
#[required_permissions("MANAGE_GUILD")]
pub struct Settings;
