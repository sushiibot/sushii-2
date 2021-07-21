use serenity::framework::standard::macros::group;

mod cases;
mod channel;
mod chat;
mod lookup;

use self::{
    cases::{ban::*, delete::*, history::*, kick::*, mute::*, note::*, reason::*, warn::*},
    channel::*,
    chat::*,
    lookup::*,
};

#[group]
#[commands(
    prune, history, ban, unban, kick, mute, listmutes, reason, unmute, warn, slowmode, deletecase,
    note
)]
#[only_in("guild")]
#[sub_groups(Lookup)]
#[required_permissions("BAN_MEMBERS")]
pub struct Moderation;

#[group]
#[commands(lookup, optin, optout)]
#[default_command(lookup)]
#[prefix("lookup")]
#[only_in("guild")]
#[required_permissions("BAN_MEMBERS")]
pub struct Lookup;
