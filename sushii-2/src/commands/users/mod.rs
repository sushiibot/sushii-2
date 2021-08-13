use serenity::framework::standard::macros::group;

mod avatar;
mod banner;
mod fishy;
mod hug;
mod rank;
mod rep;
mod userinfo;

use self::{avatar::*, banner::*, fishy::*, hug::*, rank::*, rep::*, userinfo::*};

#[group]
#[commands(avatar, banner, rank, rep, fishy, userinfo, hug)]
#[only_in("guild")]
pub struct Users;
