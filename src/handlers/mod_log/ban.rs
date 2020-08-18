use crate::model::sql::mod_log::*;
use crate::error::Result;
use crate::utils::{self, guild_config, sushii_config};
use serenity::{model::prelude::*, prelude::*};

async fn get_user_or_bot(ctx: &Context, id: Option<i64>) -> User {
    // No user provided, use bot
    if id.is_none() {
        return ctx.cache.current_user().await.into();
    }

    // Fetch from cache or http
    if let Some(user) = utils::user::get_user(&ctx, id.unwrap() as u64).await {
        return user;
    }

    // Still failed, use bot
    ctx.cache.current_user().await.into()
}

pub async fn guild_ban_addition(ctx: &Context, guild_id: &GuildId, banned_user: &User) {
    if let Err(e) = _guild_ban_addition(ctx, guild_id, banned_user).await {
        tracing::error!("Failed to handle guild_ban_addition: {}", e);
    }
}

async fn _guild_ban_addition(ctx: &Context, guild_id: &GuildId, banned_user: &User) -> Result<()> {
    // check if a ban command was used instead of discord right click ban
    // add the action to the database if not pendings
    let mut mod_log_entry = {
        let pending = ModLogEntry::get_pending_entry(&ctx, "ban", guild_id.0, banned_user.id.0).await?;

        match pending {
            Some(entry) => entry,
            None => ModLogEntry::new("ban", false, guild_id.0, &banned_user)
                .save(&ctx)
                .await?
        }
    };

    let executor_user = get_user_or_bot(&ctx, mod_log_entry.executor_id).await;

    if mod_log_entry.reason.is_none() {
        let guild_conf = guild_config::get_guild_conf_from_id(&ctx, guild_id).await;

        let prefix = match guild_conf.and_then(|c| c.prefix) {
            Some(p) => p,
            None => sushii_config::get(&ctx).await.default_prefix,
        };

        let default_reason = format!("Responsible moderator: Please use `{}reason {} [reason]` to set a reason for this case.", prefix, mod_log_entry.case_id);

        mod_log_entry.reason.replace(default_reason);
    }

    // send message
    // update db entry with message id
    // update pending to false

    Ok(())
}
