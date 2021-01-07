use serenity::framework::standard::macros::group;

mod avatar;
mod fishy;
mod rank;
mod rep;
mod userinfo;

use self::{avatar::*, fishy::*, rank::*, rep::*, userinfo::*};

#[group]
#[commands(avatar, rank, rep, fishy, userinfo)]
#[only_in("guild")]
pub struct Users;
