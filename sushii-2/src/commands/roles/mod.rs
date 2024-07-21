use serenity::framework::standard::macros::group;

pub mod default;

use self::{
    default::*,
};

#[group]
#[prefix = "roles"]
#[only_in("guild")]
#[default_command(default)]
#[required_permissions("MANAGE_GUILD")]
pub struct Roles;
