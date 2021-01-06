use serenity::framework::standard::macros::group;

mod fishy;
mod rank;
mod rep;
mod userinfo;

use self::{fishy::*, rank::*, rep::*, userinfo::*};

#[group]
#[commands(rank, rep, fishy, userinfo)]
#[only_in("guild")]
pub struct Users;
