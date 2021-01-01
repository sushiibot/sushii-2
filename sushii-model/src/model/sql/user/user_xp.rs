use serde::{Deserialize, Serialize};

// #[cfg(not(feature = "graphql"))]
// use crate::keys::DbPool;
// #[cfg(not(feature = "graphql"))]
// use serenity::{model::prelude::*, prelude::*};

#[cfg(feature = "graphql")]
use juniper::graphql_object;
#[cfg(feature = "graphql")]
use sqlx::types::Decimal;

#[cfg(feature = "graphql")]
use crate::model::user::{TimeFrame, UserLevelProgress};
#[cfg(feature = "graphql")]
use crate::{
    cursor::decode_cursor,
    model::{juniper::Context, sql::CachedUser},
};

#[cfg(feature = "graphql")]
use crate::error::Result;
use crate::model::BigInt;

#[derive(Deserialize, Serialize, sqlx::FromRow, Clone, Debug)]
pub struct UserXP {
    pub user_id: BigInt,
    /// Guild ID or None if global
    pub guild_id: Option<BigInt>,
    /// User's total XP in a time period
    pub xp: BigInt,
    /// User's gained XP in a time period, None in All time
    pub xp_diff: Option<BigInt>,
}

impl UserXP {
    /// Get guild all time ranks
    #[cfg(feature = "graphql")]
    pub async fn guild_top(
        pool: &sqlx::PgPool,
        guild_id: BigInt,
        timeframe: TimeFrame,
        first: BigInt,
        after: Option<String>,
    ) -> Result<(BigInt, Vec<UserXP>)> {
        let after = if let Some(s) = after {
            Some(decode_cursor(&s)?)
        } else {
            None
        };

        guild_top_query(pool, guild_id.0, timeframe, first.0, after).await
    }

    /// Get global all time ranks
    #[cfg(feature = "graphql")]
    pub async fn global_top(
        pool: &sqlx::PgPool,
        timeframe: TimeFrame,
        first: BigInt,
        after: Option<String>,
    ) -> Result<(BigInt, Vec<UserXP>)> {
        let after = if let Some(s) = after {
            Some(decode_cursor(&s)?)
        } else {
            None
        };

        global_top_query(pool, timeframe, first.0, after).await
    }
}

#[cfg(feature = "graphql")]
#[graphql_object(
    context = Context,
    description = "User XP in a given timeframe and given scope (guild or global)"
)]
impl UserXP {
    // Gotta do this for each field
    // https://github.com/graphql-rust/juniper/issues/553
    fn user_id(&self) -> BigInt {
        self.user_id
    }

    fn guild_id(&self) -> Option<BigInt> {
        self.guild_id
    }

    fn xp(&self) -> BigInt {
        self.xp
    }

    fn xp_diff(&self) -> Option<BigInt> {
        self.xp_diff
    }

    /// Levels gained in given timeframe, None if all time
    fn timeframe_gained_levels(&self) -> Option<BigInt> {
        self.xp_diff.map(|diff| {
            let total_prog = UserLevelProgress::from_xp(self.xp.0);
            let diff_prog = UserLevelProgress::from_xp(self.xp.0 - diff.0);

            (total_prog.level.0 - diff_prog.level.0).into()
        })
    }

    async fn user(ctx: &Context) -> Option<CachedUser> {
        ctx.cached_user_loader.load(self.user_id.0).await
    }

    fn xp_progress(&self) -> UserLevelProgress {
        UserLevelProgress::from_xp(self.xp.0)
    }
}

#[cfg(feature = "graphql")]
async fn guild_timeframe_user_count(
    pool: &sqlx::PgPool,
    guild_id: i64,
    timeframe: TimeFrame,
) -> Result<BigInt> {
    // Timeframes also match year, so that old inactive users aren't considered
    // ie. if month could match from last year, but not that significant
    match timeframe {
        TimeFrame::AllTime => sqlx::query!(
            r#"
                SELECT COUNT(*) as "total!: BigInt"
                    FROM user_levels
                    WHERE guild_id = $1
                "#,
            guild_id,
        )
        .fetch_one(pool)
        .await
        .map(|r| r.total),
        TimeFrame::Day => sqlx::query!(
            r#"
                SELECT COUNT(*) as "total!: BigInt"
                  FROM user_levels
                 WHERE guild_id = $1
                   AND EXTRACT(DOY  FROM last_msg) = EXTRACT(DOY  FROM NOW())
                   AND EXTRACT(YEAR FROM last_msg) = EXTRACT(YEAR FROM NOW())
                "#,
            guild_id,
        )
        .fetch_one(pool)
        .await
        .map(|r| r.total),
        TimeFrame::Week => sqlx::query!(
            r#"
                SELECT COUNT(*) as "total!: BigInt"
                  FROM user_levels
                 WHERE guild_id = $1
                   AND EXTRACT(WEEK FROM last_msg) = EXTRACT(WEEK FROM NOW())
                   AND EXTRACT(YEAR FROM last_msg) = EXTRACT(YEAR FROM NOW())
                "#,
            guild_id,
        )
        .fetch_one(pool)
        .await
        .map(|r| r.total),
        TimeFrame::Month => sqlx::query!(
            r#"
                SELECT COUNT(*) as "total!: BigInt"
                  FROM user_levels
                 WHERE guild_id = $1
                   AND EXTRACT(MONTH FROM last_msg) = EXTRACT(MONTH FROM NOW())
                   AND EXTRACT(YEAR  FROM last_msg) = EXTRACT(YEAR  FROM NOW())
                "#,
            guild_id,
        )
        .fetch_one(pool)
        .await
        .map(|r| r.total),
    }
    .map_err(Into::into)
}

#[cfg(feature = "graphql")]
async fn guild_timeframe_users(
    pool: &sqlx::PgPool,
    guild_id: i64,
    timeframe: TimeFrame,
    first: i64,
    after: Option<(i64, i64)>,
) -> Result<Vec<UserXP>> {
    match timeframe {
        TimeFrame::AllTime => {
            sqlx::query_as!(
                UserXP,
                // Force guild_id to be nullable since we use None for global XP
                r#"
                    SELECT user_id as "user_id: BigInt",
                           guild_id as "guild_id?: BigInt",
                           msg_all_time as "xp: BigInt",
                           NULL as "xp_diff?: BigInt"
                      FROM user_levels
                     WHERE guild_id = $1
                       AND ((msg_all_time, user_id) < ($2, $3) OR $2 IS NULL OR $3 IS NULL)
                  ORDER BY "xp: BigInt" DESC,
                           "user_id: BigInt" DESC
                     LIMIT $4
                "#,
                guild_id,
                after.map(|a| a.0), // xp
                after.map(|a| a.1), // user id
                first,
            )
            .fetch_all(pool)
            .await
        }
        TimeFrame::Day => {
            sqlx::query_as!(
                UserXP,
                r#"
                    SELECT user_id as "user_id: BigInt",
                           guild_id as "guild_id?: BigInt",
                           msg_all_time as "xp: BigInt",
                           msg_day as "xp_diff?: BigInt"
                      FROM user_levels
                     WHERE guild_id = $1
                       AND ((msg_day, user_id) < ($2, $3) OR $2 IS NULL OR $3 IS NULL)
                       AND EXTRACT(DOY  FROM last_msg) = EXTRACT(DOY  FROM NOW())
                       AND EXTRACT(YEAR FROM last_msg) = EXTRACT(YEAR FROM NOW())
                  ORDER BY "xp_diff?: BigInt" DESC,
                           "user_id: BigInt" DESC
                     LIMIT $4
                "#,
                guild_id,
                after.map(|a| a.0), // xp
                after.map(|a| a.1), // user id
                first,
            )
            .fetch_all(pool)
            .await
        }
        TimeFrame::Week => {
            sqlx::query_as!(
                UserXP,
                r#"
                    SELECT user_id as "user_id: BigInt",
                           guild_id as "guild_id?: BigInt",
                           msg_all_time as "xp: BigInt",
                           msg_week as "xp_diff?: BigInt"
                      FROM user_levels
                     WHERE guild_id = $1
                       AND ((msg_week, user_id) < ($2, $3) OR $2 IS NULL OR $3 IS NULL)
                       AND EXTRACT(WEEK FROM last_msg) = EXTRACT(WEEK FROM NOW())
                       AND EXTRACT(YEAR FROM last_msg) = EXTRACT(YEAR FROM NOW())
                  ORDER BY "xp_diff?: BigInt" DESC,
                           "user_id: BigInt" DESC
                     LIMIT $4
                "#,
                guild_id,
                after.map(|a| a.0), // xp
                after.map(|a| a.1), // user id
                first,
            )
            .fetch_all(pool)
            .await
        }
        TimeFrame::Month => {
            sqlx::query_as!(
                UserXP,
                r#"
                    SELECT user_id as "user_id: BigInt",
                           guild_id as "guild_id?: BigInt",
                           msg_all_time as "xp: BigInt",
                           msg_month as "xp_diff?: BigInt"
                      FROM user_levels
                     WHERE guild_id = $1
                       AND ((msg_month, user_id) < ($2, $3) OR $2 IS NULL OR $3 IS NULL)
                       AND EXTRACT(MONTH FROM last_msg) = EXTRACT(MONTH FROM NOW())
                       AND EXTRACT(YEAR  FROM last_msg) = EXTRACT(YEAR  FROM NOW())
                  ORDER BY "xp_diff?: BigInt" DESC,
                           "user_id: BigInt" DESC
                     LIMIT $4
                "#,
                guild_id,
                after.map(|a| a.0), // xp
                after.map(|a| a.1), // user id
                first,
            )
            .fetch_all(pool)
            .await
        }
    }
    .map_err(Into::into)
}

#[cfg(feature = "graphql")]
async fn guild_top_query(
    pool: &sqlx::PgPool,
    guild_id: i64,
    timeframe: TimeFrame,
    first: i64,
    after: Option<(i64, i64)>,
) -> Result<(BigInt, Vec<UserXP>)> {
    let (total, users) = tokio::join!(
        guild_timeframe_user_count(pool, guild_id, timeframe),
        guild_timeframe_users(pool, guild_id, timeframe, first, after),
    );

    Ok((total?, users?))
}

#[cfg(feature = "graphql")]
async fn global_timeframe_user_count(pool: &sqlx::PgPool, timeframe: TimeFrame) -> Result<BigInt> {
    // Timeframes also match year, so that old inactive users aren't considered
    // ie. if month could match from last year, but not that significant
    match timeframe {
        TimeFrame::AllTime => sqlx::query!(
            r#"
                  SELECT COUNT(DISTINCT user_id) as "total!: BigInt"
                    FROM user_levels
                "#,
        )
        .fetch_one(pool)
        .await
        .map(|r| r.total),
        TimeFrame::Day => sqlx::query!(
            r#"
                  SELECT COUNT(DISTINCT user_id) as "total!: BigInt"
                    FROM user_levels
                   WHERE EXTRACT(DOY  FROM last_msg) = EXTRACT(DOY  FROM NOW())
                     AND EXTRACT(YEAR FROM last_msg) = EXTRACT(YEAR FROM NOW())
                "#,
        )
        .fetch_one(pool)
        .await
        .map(|r| r.total),
        TimeFrame::Week => sqlx::query!(
            r#"
                  SELECT COUNT(DISTINCT user_id) as "total!: BigInt"
                    FROM user_levels
                   WHERE EXTRACT(WEEK FROM last_msg) = EXTRACT(WEEK FROM NOW())
                     AND EXTRACT(YEAR FROM last_msg) = EXTRACT(YEAR FROM NOW())
                "#,
        )
        .fetch_one(pool)
        .await
        .map(|r| r.total),
        TimeFrame::Month => sqlx::query!(
            r#"
                  SELECT COUNT(DISTINCT user_id) as "total!: BigInt"
                    FROM user_levels
                   WHERE EXTRACT(MONTH FROM last_msg) = EXTRACT(MONTH FROM NOW())
                     AND EXTRACT(YEAR  FROM last_msg) = EXTRACT(YEAR  FROM NOW())
                "#,
        )
        .fetch_one(pool)
        .await
        .map(|r| r.total),
    }
    .map_err(Into::into)
}

#[cfg(feature = "graphql")]
async fn global_timeframe_users(
    pool: &sqlx::PgPool,
    timeframe: TimeFrame,
    first: i64,
    after: Option<(i64, i64)>,
) -> Result<Vec<UserXP>> {
    match timeframe {
        TimeFrame::AllTime => {
            tracing::warn!("all time query");
            sqlx::query_as!(
                UserXP,
                r#"
                    SELECT user_id as "user_id: BigInt",
                           NULL as "guild_id?: BigInt",
                           CAST(SUM(msg_all_time) AS BIGINT) AS "xp!: BigInt",
                           NULL as "xp_diff?: BigInt"
                      FROM user_levels
                  GROUP BY user_id
                    HAVING ((SUM(msg_all_time), user_id) < ($1, $2) OR $1 IS NULL OR $2 IS NULL)
                  ORDER BY "xp!: BigInt" DESC,
                           "user_id: BigInt" DESC
                    LIMIT $3
                "#,
                after.map(|a| Decimal::from(a.0)), // xp
                after.map(|a| a.1),                // user id
                first,
            )
            .fetch_all(pool)
            .await
        }
        TimeFrame::Day => {
            sqlx::query_as!(
                UserXP,
                r#"
                    SELECT user_id as "user_id: BigInt",
                           NULL as "guild_id?: BigInt",
                           CAST(SUM(msg_all_time) AS BIGINT) AS "xp!: BigInt",
                           CAST(SUM(msg_day) AS BIGINT) AS "xp_diff?: BigInt"
                      FROM user_levels
                     WHERE EXTRACT(DOY  FROM last_msg) = EXTRACT(DOY  FROM NOW())
                       AND EXTRACT(YEAR FROM last_msg) = EXTRACT(YEAR FROM NOW())
                  GROUP BY user_id
                        -- after
                    HAVING ((SUM(msg_day), user_id) < ($1, $2) OR $1 IS NULL OR $2 IS NULL)
                  ORDER BY "xp!: BigInt" DESC,
                           "user_id: BigInt" DESC
                    LIMIT $3
                "#,
                after.map(|a| Decimal::from(a.0)), // xp
                after.map(|a| a.1),                // user id
                first,
            )
            .fetch_all(pool)
            .await
        }
        TimeFrame::Week => {
            sqlx::query_as!(
                UserXP,
                r#"
                    SELECT user_id as "user_id: BigInt",
                           NULL as "guild_id?: BigInt",
                           CAST(SUM(msg_all_time) AS BIGINT) AS "xp!: BigInt",
                           CAST(SUM(msg_week) AS BIGINT) AS "xp_diff?: BigInt"
                      FROM user_levels
                     WHERE EXTRACT(WEEK FROM last_msg) = EXTRACT(WEEK FROM NOW())
                       AND EXTRACT(YEAR FROM last_msg) = EXTRACT(YEAR FROM NOW())
                  GROUP BY user_id
                        -- after
                    HAVING ((SUM(msg_week), user_id) < ($1, $2) OR $1 IS NULL OR $2 IS NULL)
                  ORDER BY "xp!: BigInt" DESC,
                           "user_id: BigInt" DESC
                    LIMIT $3
                "#,
                after.map(|a| Decimal::from(a.0)), // xp
                after.map(|a| a.1),                // user id
                first,
            )
            .fetch_all(pool)
            .await
        }
        TimeFrame::Month => {
            sqlx::query_as!(
                UserXP,
                r#"
                    SELECT user_id as "user_id: BigInt",
                           NULL as "guild_id?: BigInt",
                           CAST(SUM(msg_all_time) AS BIGINT) AS "xp!: BigInt",
                           CAST(SUM(msg_month) AS BIGINT) AS "xp_diff?: BigInt"
                      FROM user_levels
                     WHERE EXTRACT(MONTH FROM last_msg) = EXTRACT(MONTH FROM NOW())
                       AND EXTRACT(YEAR  FROM last_msg) = EXTRACT(YEAR  FROM NOW())
                  GROUP BY user_id
                        -- after
                    HAVING ((SUM(msg_month), user_id) < ($1, $2) OR $1 IS NULL OR $2 IS NULL)
                  ORDER BY "xp!: BigInt" DESC,
                           "user_id: BigInt" DESC
                    LIMIT $3
                "#,
                after.map(|a| Decimal::from(a.0)), // xp
                after.map(|a| a.1),                // user id
                first,
            )
            .fetch_all(pool)
            .await
        }
    }
    .map_err(Into::into)
}

#[cfg(feature = "graphql")]
async fn global_top_query(
    pool: &sqlx::PgPool,
    timeframe: TimeFrame,
    first: i64,
    after: Option<(i64, i64)>,
) -> Result<(BigInt, Vec<UserXP>)> {
    let (total, users) = tokio::join!(
        global_timeframe_user_count(pool, timeframe),
        global_timeframe_users(pool, timeframe, first, after),
    );

    Ok((total?, users?))
}
