use serenity::framework::standard::macros::group;

pub mod settings;

use settings::*;

#[group]
#[commands(prefix)]
#[required_permissions("BAN_MEMBERS")]
pub struct Moderation;
