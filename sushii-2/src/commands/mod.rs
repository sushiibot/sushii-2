use serenity::framework::standard::macros::group;

pub mod guild;
pub mod help;
pub mod lastfm;
pub mod meta;
pub mod moderation;
pub mod notifications;
pub mod owner;
pub mod patreon;
pub mod prefix;
pub mod reminders;
pub mod roles;
pub mod settings;
pub mod tags;
pub mod users;

use self::{help::*, meta::*, owner::*, prefix::*};

#[group]
#[commands(quit, say, listservers)]
pub struct Owner;

#[group]
#[commands(prefix, ping, patreon, invite, about, help)]
pub struct Meta;
