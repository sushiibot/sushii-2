use serenity::framework::standard::macros::group;

mod avatar;
mod fishy;
mod rank;
mod rep;
mod userinfo;
mod hug;

use self::{avatar::*, fishy::*, rank::*, rep::*, userinfo::*, hug::*};

#[group]
#[commands(avatar, rank, rep, fishy, userinfo, hug)]
#[only_in("guild")]
pub struct Users;
