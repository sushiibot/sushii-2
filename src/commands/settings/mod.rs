use serenity::framework::standard::macros::group;

mod list;
mod mod_log;

use self::{list::*, mod_log::*};

#[group]
#[commands(list, modlog)]
#[description("Guild settings, requires MANAGE_GUILD permissions")]
#[prefix = "settings"]
#[only_in("guild")]
#[required_permissions("MANAGE_GUILD")]
pub struct Settings;
