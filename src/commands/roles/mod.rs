use serenity::framework::standard::macros::group;

pub mod get_roles;
pub mod set_roles;
pub mod set_roles_channel;

use self::{get_roles::*, set_roles::*, set_roles_channel::*};

#[group]
#[commands(set, get, set_channel)]
#[prefix = "roles"]
#[only_in("guild")]
#[required_permissions("MANAGE_GUILD")]
pub struct Roles;
