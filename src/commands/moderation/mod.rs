use serenity::framework::standard::macros::group;

pub mod chat;
pub mod settings;

use chat::*;
use settings::*;

#[group]
#[commands(prefix, prune, settings)]
#[only_in("guild")]
#[required_permissions("BAN_MEMBERS")]
pub struct Moderation;
