use serenity::framework::standard::macros::group;

mod list;
mod mod_log;
mod mute_role;

use self::{list::*, mod_log::*, mute_role::*};

#[group]
#[commands(list, modlog, muterole)]
#[description("Guild settings, requires MANAGE_GUILD permissions")]
#[prefix = "settings"]
#[only_in("guild")]
#[required_permissions("MANAGE_GUILD")]
pub struct Settings;
