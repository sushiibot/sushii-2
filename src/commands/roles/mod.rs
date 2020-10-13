use serenity::framework::standard::macros::group;

pub mod default;
pub mod get_roles;
pub mod list_role_ids;
pub mod send_info;
pub mod set_roles;
pub mod set_roles_channel;

use self::{
    default::*, get_roles::*, list_role_ids::*, send_info::*, set_roles::*, set_roles_channel::*,
};

#[group]
#[commands(set, get, setchannel, listids, sendinfo)]
#[prefix = "roles"]
#[only_in("guild")]
#[default_command(default)]
#[required_permissions("MANAGE_GUILD")]
pub struct Roles;
