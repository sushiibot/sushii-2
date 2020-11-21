use serenity::framework::standard::macros::group;

pub mod rank;

use self::{rank::*};

#[group]
#[commands(
    rank
)]
#[only_in("guild")]
pub struct Users;
