use serenity::framework::standard::macros::group;

mod cases;
mod channel;
mod chat;
mod lookup;

use self::{
    cases::{ban::*, delete::*, history::*, kick::*, mute::*, reason::*, warn::*},
    channel::*,
    chat::*,
    lookup::*,
};

#[group]
#[commands(
    prune, history, ban, unban, kick, mute, listmutes, reason, unmute, warn, slowmode, deletecase,
    lookup
)]
#[only_in("guild")]
#[required_permissions("BAN_MEMBERS")]
pub struct Moderation;
