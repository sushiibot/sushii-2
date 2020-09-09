use serenity::framework::standard::macros::group;

pub mod cases;
pub mod chat;

use self::{
    cases::{ban::*, history::*, kick::*, mute::*},
    chat::*,
};

#[group]
#[commands(prune, history, ban, unban, kick, mute, unmute)]
#[only_in("guild")]
#[required_permissions("BAN_MEMBERS")]
pub struct Moderation;
