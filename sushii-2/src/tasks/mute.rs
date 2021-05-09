use serenity::model::prelude::*;
use serenity::prelude::*;

use crate::error::Result;
use crate::model::sql::*;

pub async fn check_pending_unmutes(ctx: &Context) -> Result<()> {
    let expired_mutes = Mute::get_expired(&ctx).await?;
    tracing::debug!("Found {} expired mute entries", expired_mutes.len());

    for mute in expired_mutes {
        // Don't use ? since we want to try the rest of the mute entries instead
        // of just stopping on any error
        if let Err(e) = unmute_member(&ctx, &mute).await {
            tracing::error!(?mute, "Failed to unmute member: {}", e);
        }
    }

    Ok(())
}

fn is_member_unknown_error(err: &serenity::Error) -> bool {
    match err {
        serenity::Error::Http(e) => {
            // Dereference serenity error then deref Box, then borrow it to not take ownership lol
            match &**e {
                serenity::http::error::Error::UnsuccessfulRequest(
                    serenity::http::error::ErrorResponse { error, .. },
                ) => {
                    tracing::error!("Failed to fetch member: {:#?}", error.errors);

                    // https://discord.com/developers/docs/topics/opcodes-and-status-codes#json
                    // Member not found, Unknown Member
                    // Meaning they are not in the guild
                    error.code == 10007
                }
                _ => false,
            }
        }
        _ => false,
    }
}

pub async fn unmute_member(ctx: &Context, mute: &Mute) -> Result<()> {
    let guild_id = GuildId(mute.guild_id as u64);

    let failure_id = format!(
        "app_public.mutes:{}:{}",
        mute.guild_id as u64, mute.user_id as u64
    );

    // First check if this failed too many times
    let failure = match Failure::from_id(ctx, &failure_id).await? {
        Some(f) => {
            // Too many attempts, delete and exit
            if f.exceeded_attempts() {
                tracing::info!(
                    ?mute,
                    "Mute fail count exceeded max attempts ({}), deleting",
                    f.attempt_count
                );
                mute.delete(ctx).await?;

                // Also delete failure since we don't need to keep track anymore
                f.delete(ctx).await?;

                return Ok(());
            }

            // Failed attempt before, skip until next attempt time
            // May accumulate for certain amount of time (25 failures)
            if !f.should_attempt() {
                return Ok(());
            }

            Some(f)
        }
        None => None,
    };

    // Possibly inefficient here since there can be the same guild config
    // fetched here, but it's likely that there aren't many entries at a single
    // time since the check happens every 10 seconds
    let guild_conf = match GuildConfig::from_id(&ctx, &guild_id).await? {
        Some(c) => c,
        None => {
            tracing::warn!("Guild config not found handling mute expirey");
            return Ok(());
        }
    };

    let mute_role = match guild_conf.mute_role {
        Some(role) => role as u64,
        None => {
            tracing::warn!(
                ?guild_conf,
                "Guild mute role not found handling mute expirey"
            );
            return Ok(());
        }
    };

    let user = UserId(mute.user_id as u64).to_user(&ctx).await?;

    let member = match guild_id.member(&ctx, mute.user_id as u64).await {
        Ok(m) => Some(m),
        Err(e) => {
            // Some other error failed, member could still be in the guild
            if !is_member_unknown_error(&e) {
                // Inc failures
                let mut failure = failure.unwrap_or_else(|| Failure::new(failure_id));
                failure.inc();
                failure = failure.save(ctx).await?;

                tracing::warn!(
                    ?mute,
                    "Mute member fetch for guild {} user {} failed (attempt {}), trying again at {}",
                    mute.guild_id as u64,
                    mute.user_id as u64,
                    failure.attempt_count,
                    failure.next_attempt
                );

                return Err(e.into());
            }

            // Member is not found in existing guild
            None
        }
    };

    let mut reason = format!(
        "Automated Unmute: Mute expired (Duration: {}).",
        mute.get_human_duration().unwrap_or_else(|| "N/A".into()),
    );

    // TODO: Send modlog entry if member not in guild since it only sends when modifying user roles
    if member.is_none() {
        reason.push_str(" User is currently not in guild and will not be muted on re-join.");
    }

    ModLogEntry::new("unmute", true, guild_id.0, user.id.0, &user.tag())
        .reason(&Some(reason))
        .save(&ctx)
        .await?;

    if let Some(mut m) = member {
        if let Err(e) = m.remove_role(&ctx, mute_role).await {
            let mut failure = failure.unwrap_or_else(|| Failure::new(failure_id));
            failure.inc();
            failure = failure.save(ctx).await?;

            tracing::warn!(
                ?mute,
                "Mute role remove for guild {} user {} failed (attempt {}), trying again at {}",
                mute.guild_id as u64,
                mute.user_id as u64,
                failure.attempt_count,
                failure.next_attempt
            );

            return Err(e.into());
        }
    }

    mute.delete(&ctx).await?;
    if let Some(failure) = failure {
        failure.delete(ctx).await?;
    }

    Ok(())
}
