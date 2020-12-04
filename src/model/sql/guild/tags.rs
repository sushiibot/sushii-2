use chrono::{naive::NaiveDateTime, offset::Utc};
use serde::{Deserialize, Serialize};
use serenity::async_trait;
use serenity::model::prelude::*;
use serenity::prelude::*;

use crate::prelude::*;

#[derive(Deserialize, Serialize, sqlx::FromRow, Debug)]
pub struct Tag {
    pub owner_id: i64,
    pub guild_id: i64,
    pub tag_name: String,
    pub content: String,
    pub use_count: i64,
    pub created: NaiveDateTime,
}

impl Tag {
    pub fn new(owner_id: UserId, guild_id: GuildId, tag_name: &str, content: &str) -> Self {
        let created = Utc::now().naive_local();

        Self {
            owner_id: i64::from(owner_id),
            guild_id: i64::from(guild_id),
            tag_name: tag_name.into(),
            content: content.into(),
            use_count: 0,
            created,
        }
    }

    pub fn edit(mut self, content: &str) -> Self {
        self.content = content.to_string();
        self
    }
}

#[async_trait]
pub trait TagDb {
    async fn from_name(ctx: &Context, tag_name: &str, guild_id: GuildId) -> Result<Option<Tag>>;
    async fn random(ctx: &Context, guild_id: GuildId) -> Result<Option<Tag>>;
    /// Searches for a given substring
    async fn search(
        ctx: &Context,
        guild_id: GuildId,
        query: &str,
        count: i64,
        offset: Option<&str>,
    ) -> Result<Vec<Tag>>;

    /// Get paginated list of all tags
    async fn get_page(
        ctx: &Context,
        guild_id: GuildId,
        count: i64,
        offset: Option<&str>,
    ) -> Result<Vec<Tag>>;
    async fn get_count(ctx: &Context, guild_id: GuildId) -> Result<i64>;

    async fn can_edit(&self, ctx: &Context, member: &Member) -> Result<bool>;
    async fn rename(&mut self, ctx: &Context, tag_name: &str) -> Result<bool>;
    async fn save(&self, ctx: &Context) -> Result<Tag>;
    async fn delete(&self, ctx: &Context) -> Result<()>;
}

#[async_trait]
impl TagDb for Tag {
    async fn from_name(ctx: &Context, tag_name: &str, guild_id: GuildId) -> Result<Option<Tag>> {
        let pool = ctx.data.read().await.get::<DbPool>().cloned().unwrap();

        from_name_query(&pool, tag_name, guild_id).await
    }

    async fn random(ctx: &Context, guild_id: GuildId) -> Result<Option<Tag>> {
        let pool = ctx.data.read().await.get::<DbPool>().cloned().unwrap();

        random_query(&pool, guild_id).await
    }

    async fn search(
        ctx: &Context,
        guild_id: GuildId,
        query: &str,
        count: i64,
        offset: Option<&str>,
    ) -> Result<Vec<Tag>> {
        let pool = ctx.data.read().await.get::<DbPool>().cloned().unwrap();

        search_query(&pool, guild_id, query, count, offset).await
    }

    /// Get paginated list of all tags
    async fn get_page(
        ctx: &Context,
        guild_id: GuildId,
        count: i64,
        offset: Option<&str>,
    ) -> Result<Vec<Tag>> {
        let pool = ctx.data.read().await.get::<DbPool>().cloned().unwrap();

        get_page_query(&pool, guild_id, count, offset).await
    }

    /// Get total number of tags in a given guild
    async fn get_count(ctx: &Context, guild_id: GuildId) -> Result<i64> {
        let pool = ctx.data.read().await.get::<DbPool>().cloned().unwrap();

        get_count_query(&pool, guild_id).await
    }

    async fn can_edit(&self, ctx: &Context, member: &Member) -> Result<bool> {
        Ok(i64::from(member.user.id) == self.owner_id
            || member.permissions(&ctx).await?.manage_guild())
    }

    /// Renames this tag, returns false if there is already a tag with the desired name
    async fn rename(&mut self, ctx: &Context, tag_name: &str) -> Result<bool> {
        // Check for existing tag
        if Self::from_name(&ctx, tag_name, GuildId(self.guild_id as u64))
            .await?
            .is_some()
        {
            return Ok(false);
        }

        // New name is unused, we can rename it now
        self.tag_name = tag_name.to_string();

        Ok(true)
    }

    async fn save(&self, ctx: &Context) -> Result<Tag> {
        let pool = ctx.data.read().await.get::<DbPool>().cloned().unwrap();

        upsert_query(&pool, &self).await
    }

    async fn delete(&self, ctx: &Context) -> Result<()> {
        let pool = ctx.data.read().await.get::<DbPool>().cloned().unwrap();

        delete_query(&pool, &self).await
    }
}

async fn from_name_query(
    pool: &sqlx::PgPool,
    tag_name: &str,
    guild_id: GuildId,
) -> Result<Option<Tag>> {
    sqlx::query_as!(
        Tag,
        r#"
            SELECT *
              FROM tags
             WHERE tag_name = $1
               AND guild_id = $2
        "#,
        tag_name,
        i64::from(guild_id),
    )
    .fetch_optional(pool)
    .await
    .map_err(Into::into)
}

async fn random_query(pool: &sqlx::PgPool, guild_id: GuildId) -> Result<Option<Tag>> {
    sqlx::query_as!(
        Tag,
        r#"
            SELECT *
              FROM tags
             WHERE guild_id = $1
            OFFSET floor(random() * (
                       SELECT COUNT(*)
                         FROM tags))
             LIMIT 1
        "#,
        i64::from(guild_id),
    )
    .fetch_optional(pool)
    .await
    .map_err(Into::into)
}

async fn search_query(
    pool: &sqlx::PgPool,
    guild_id: GuildId,
    query: &str,
    count: i64,
    offset: Option<&str>,
) -> Result<Vec<Tag>> {
    sqlx::query_as!(
        Tag,
        // Should have a trigram index for this search, B-tree indexes can't
        // search patterns that aren't left-anchored
        r#"
              SELECT *
                FROM tags
               WHERE guild_id = $1
                 AND tag_name = '%' || $2 || '%'
                 AND (tag_name > $3 OR $3 IS NULL)
            ORDER BY tag_name ASC
               LIMIT $4
        "#,
        i64::from(guild_id),
        query,
        offset,
        count,
    )
    .fetch_all(pool)
    .await
    .map_err(Into::into)
}

async fn get_page_query(
    pool: &sqlx::PgPool,
    guild_id: GuildId,
    count: i64,
    offset: Option<&str>,
) -> Result<Vec<Tag>> {
    sqlx::query_as!(
        Tag,
        r#"
              SELECT *
                FROM tags
               WHERE guild_id = $1
                 AND (tag_name > $2 OR $2 IS NULL)
            ORDER BY tag_name ASC
               LIMIT $3
        "#,
        i64::from(guild_id),
        offset,
        count,
    )
    .fetch_all(pool)
    .await
    .map_err(Into::into)
}

async fn get_count_query(pool: &sqlx::PgPool, guild_id: GuildId) -> Result<i64> {
    sqlx::query!(
        r#"
              SELECT COUNT(*) as "count!"
                FROM tags
               WHERE guild_id = $1
        "#,
        i64::from(guild_id),
    )
    .fetch_one(pool)
    .await
    .map(|r| r.count)
    .map_err(Into::into)
}

async fn upsert_query(pool: &sqlx::PgPool, tag: &Tag) -> Result<Tag> {
    sqlx::query_as!(
        Tag,
        r#"
        INSERT INTO tags (owner_id, guild_id, tag_name, content, use_count, created)
             VALUES ($1, $2, $3, $4, $5, $6)
        ON CONFLICT (guild_id, tag_name)
          DO UPDATE
                SET tag_name = $3,
                    content = $4,
                    use_count = $5
          RETURNING *
        "#,
        tag.owner_id,
        tag.guild_id,
        tag.tag_name,
        tag.content,
        tag.use_count,
        tag.created,
    )
    .fetch_one(pool)
    .await
    .map_err(Into::into)
}

async fn delete_query(pool: &sqlx::PgPool, tag: &Tag) -> Result<()> {
    sqlx::query!(
        r#"
            DELETE FROM tags
                  WHERE guild_id = $1
                    AND tag_name = $2
        "#,
        tag.guild_id,
        tag.tag_name,
    )
    .execute(pool)
    .await?;

    Ok(())
}
