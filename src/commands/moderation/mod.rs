use serenity::framework::standard::macros::group;

pub mod cases;
pub mod chat;
pub mod settings;

use self::{
    cases::{ban::*, history::*, kick::*, mute::*},
    chat::*,
    settings::*,
};

#[group]
#[commands(prefix, prune, settings, history, ban, unban, kick, mute, unmute)]
#[only_in("guild")]
#[required_permissions("BAN_MEMBERS")]
pub struct Moderation;
