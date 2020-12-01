use serenity::framework::standard::macros::group;

pub mod fishy;
pub mod rank;
pub mod rep;

use self::{fishy::*, rank::*, rep::*};

#[group]
#[commands(rank, rep, fishy)]
#[only_in("guild")]
pub struct Users;
