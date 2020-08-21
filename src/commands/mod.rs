use serenity::framework::standard::macros::group;

pub mod help;
pub mod meta;
pub mod moderation;
pub mod owner;
pub mod roles;

use self::meta::*;
use self::owner::*;

#[group]
#[commands(quit)]
pub struct Owner;

#[group]
#[commands(ping)]
pub struct Meta;
