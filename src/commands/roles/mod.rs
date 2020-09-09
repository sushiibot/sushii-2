use serenity::framework::standard::macros::group;

pub mod get_roles;
pub mod set_roles;
pub mod set_roles_channel;
pub mod list_role_ids;

use self::{get_roles::*, set_roles::*, set_roles_channel::*, list_role_ids::*};

#[group]
#[commands(set, get, set_channel, listids)]
#[prefix = "roles"]
#[only_in("guild")]
#[required_permissions("MANAGE_GUILD")]
pub struct Roles;
