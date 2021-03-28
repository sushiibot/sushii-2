use serenity::framework::standard::macros::group;

mod avatar;
mod fishy;
mod hug;
mod rank;
mod rep;
mod userinfo;

use self::{avatar::*, fishy::*, hug::*, rank::*, rep::*, userinfo::*};

#[group]
#[commands(avatar, rank, rep, fishy, userinfo, hug)]
#[only_in("guild")]
pub struct Users;
