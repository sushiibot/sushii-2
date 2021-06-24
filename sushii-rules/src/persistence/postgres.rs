use crate::model::{constraint::*, Action, Condition, Rule, RuleSet, Trigger};
use crate::error::Result;

pub async fn get_guild_rule_sets(pool: &sqlx::PgPool, guild_id: u64) -> Result<Vec<RuleSet>> {
    sqlx::query_as!(
        RuleSet,
        "select id,
               guild_id,
               name,
               description,
               enabled,
               editable,
               author,
               category,
               config,
               array(select *
                  from app_public.guild_rules
                 where set_id = s.id
                   and guild_id = $1) as rules
          from app_public.guild_rule_sets s
         where guild_id = $1
        ",
        guild_id as i64,
    )
    .fetch_all(pool)
    .await
    .map_err(Into::into)
}
