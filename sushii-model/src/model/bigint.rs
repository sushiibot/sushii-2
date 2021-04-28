use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Clone, Copy, Debug, sqlx::Type)]
#[sqlx(transparent)]
pub struct BigInt(pub i64);

impl From<BigInt> for u64 {
    fn from(id: BigInt) -> u64 {
        id.0 as u64
    }
}

impl From<BigInt> for i64 {
    fn from(id: BigInt) -> i64 {
        id.0 as i64
    }
}

use serenity::model::id::*;

impl From<i64> for BigInt {
    fn from(num: i64) -> BigInt {
        BigInt(num)
    }
}

impl From<u64> for BigInt {
    fn from(num: u64) -> BigInt {
        BigInt(num as i64)
    }
}

impl From<UserId> for BigInt {
    fn from(id: UserId) -> BigInt {
        BigInt(id.0 as i64)
    }
}

impl From<GuildId> for BigInt {
    fn from(id: GuildId) -> BigInt {
        BigInt(id.0 as i64)
    }
}
