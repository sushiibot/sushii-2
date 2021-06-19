use serde::{Deserialize, Serialize};
use serenity::model::prelude::*;
use serenity::model::guild::Ban;
use chrono::NaiveDateTime;

use crate::prelude::*;

#[derive(Deserialize, Serialize, sqlx::FromRow, Clone, Debug)]
pub struct GuildBan {
    pub guild_id: i64,
    pub user_id: i64,
    /// Sushii latest reason fetched from mod cases
    pub reason: Option<String>,
    pub action_time: Option<NaiveDateTime>,
}

impl GuildBan {
    pub async fn update_guild_bans(
        pool: &sqlx::PgPool,
        guild_id: GuildId,
        bans: &[Ban],
    ) -> Result<()> {
        update_guild_bans_query(pool, guild_id, bans).await
    }

    pub async fn add_ban<'a, E: sqlx::Executor<'a, Database = sqlx::Postgres>>(
        exec: E,
        guild_id: GuildId,
        user_id: UserId,
    ) -> Result<()> {
        sqlx::query!(
            r#"
            INSERT INTO app_public.guild_bans (guild_id, user_id)
                 VALUES ($1, $2)
            ON CONFLICT (guild_id, user_id)
                        DO NOTHING
            "#,
            guild_id.0 as i64,
            user_id.0 as i64
        )
        .execute(exec)
        .await?;

        Ok(())
    }

    pub async fn remove_ban<'a, E: sqlx::Executor<'a, Database = sqlx::Postgres>>(
        exec: E,
        guild_id: GuildId,
        user_id: UserId,
    ) -> Result<()> {
        sqlx::query!(
            r#"
                DELETE FROM app_public.guild_bans
                      WHERE guild_id = $1
                        AND user_id = $2
            "#,
            guild_id.0 as i64,
            user_id.0 as i64,
        )
        .execute(exec)
        .await?;

        Ok(())
    }

    pub async fn lookup_user_id<'a, E: sqlx::Executor<'a, Database = sqlx::Postgres>>(
        exec: E,
        user_id: UserId,
    ) -> Result<Vec<GuildBan>> {
        sqlx::query_as!(
            GuildBan,
            r#"
                  SELECT distinct on (guild_id)
                         bans.guild_id,
                         bans.user_id,
                         reason,
                         action_time as "action_time: Option<NaiveDateTime>"
                    FROM app_public.guild_bans bans
                         LEFT JOIN app_public.mod_logs logs
                                ON bans.guild_id = logs.guild_id
                               AND bans.user_id  = logs.user_id
                   WHERE bans.user_id = $1
                     AND (action IS NULL OR action = 'ban')
                ORDER BY guild_id, action_time desc
            "#,
            user_id.0 as i64
        )
        .fetch_all(exec)
        .await
        .map_err(Into::into)
    }
}

async fn update_guild_bans_query(
        pool: &sqlx::PgPool,
        guild_id: GuildId,
        bans: &[Ban],
) -> Result<()> {
    let mut conn = pool.begin().await?;

    sqlx::query!(
        r#"
        INSERT INTO app_public.guild_bans (guild_id, user_id)
             SELECT $1, user_id
               FROM unnest($2::bigint[]) as ids(user_id)
        ON CONFLICT (guild_id, user_id)
                    DO NOTHING
        "#,
        guild_id.0 as i64,
        &bans.iter().map(|ban| ban.user.id.0 as i64).collect::<Vec<_>>(),
    )
    .execute(&mut conn)
    .await?;

    sqlx::query!(
        r#"
            DELETE FROM app_public.guild_bans
                  WHERE guild_id = $1
                    AND NOT user_id = ANY($2::bigint[])
        "#,
        guild_id.0 as i64,
        &bans.iter().map(|ban| ban.user.id.0 as i64).collect::<Vec<_>>(),
    )
    .execute(&mut conn)
    .await?;

    conn.commit().await?;

    Ok(())
}
