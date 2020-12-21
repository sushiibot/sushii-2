use serenity::framework::standard::macros::group;

pub mod guild;
pub mod help;
pub mod lastfm;
pub mod meta;
pub mod moderation;
pub mod owner;
pub mod prefix;
pub mod roles;
pub mod settings;
pub mod tags;
pub mod users;

use self::{help::*, meta::*, owner::*, prefix::*};

#[group]
#[commands(quit)]
pub struct Owner;

#[group]
#[commands(prefix, ping, invite, about, help)]
pub struct Meta;