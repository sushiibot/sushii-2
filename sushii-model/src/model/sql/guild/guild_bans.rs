use serde::{Deserialize, Serialize};
use serenity::model::prelude::*;
use serenity::prelude::*;
use serenity::utils::parse_channel;
use std::convert::TryFrom;
use std::fmt;
use std::time::Duration;

use crate::prelude::*;

#[derive(Deserialize, Default, Serialize, sqlx::FromRow, Clone, Debug)]
pub struct GuildBan {
    guild_id: i64,
    user_id: i64,
    reason: Option<String>,
}
