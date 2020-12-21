use serenity::framework::standard::macros::group;

mod serverinfo;

use self::serverinfo::*;

#[group]
#[commands(serverinfo)]
pub struct Guild;
