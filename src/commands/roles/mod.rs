use serenity::framework::standard::macros::group;

pub mod get_roles;
pub mod set_roles;

use self::{get_roles::*, set_roles::*};

#[group]
#[commands(set, get)]
#[prefix = "roles"]
#[only_in("guild")]
#[required_permissions("MANAGE_GUILD")]
pub struct Roles;
