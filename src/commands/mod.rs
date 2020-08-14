use serenity::framework::standard::macros::group;

pub mod groups;

pub mod meta;
pub mod moderation;
pub mod owner;

use self::meta::*;
use self::owner::*;

#[group]
#[commands(quit)]
pub struct Owner;

#[group]
#[commands(ping)]
pub struct Meta;
