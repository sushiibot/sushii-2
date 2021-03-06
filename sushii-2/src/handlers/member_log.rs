use chrono::{Duration, Utc};
use serenity::{model::prelude::*, prelude::*};
use std::fmt::Write;

use crate::error::Result;
use crate::model::sql::*;

pub async fn guild_member_addition(ctx: &Context, guild_id: &GuildId, member: &Member) {
    if let Err(e) = _guild_member_addition(&ctx, &guild_id, &member).await {
        tracing::error!("Failed to handle guild_member_addition: {}", e);
    }
}

#[tracing::instrument(skip(ctx))]
async fn _guild_member_addition(ctx: &Context, guild_id: &GuildId, member: &Member) -> Result<()> {
    let mut guild_conf = match GuildConfig::from_id(&ctx, &member.guild_id).await? {
        Some(c) => c,
        None => {
            tracing::error!(?member.guild_id, ?member, "No guild config found while handling mute guild_member_addition");
            return Ok(());
        }
    };

    if !guild_conf.log_member_enabled {
        return Ok(());
    }

    let member_log_channel = match guild_conf.log_member {
        Some(id) => ChannelId(id as u64),
        None => return Ok(()),
    };

    let mut desc = String::new();

    write!(desc, "{} joined", member.user.mention())?;

    if let Some(member_num) = guild_id.to_guild_cached(&ctx).await.map(|g| g.member_count) {
        write!(desc, " (Member #{})", member_num)?;
    }

    let now = Utc::now();
    // Truncate to minutes
    let age = Duration::minutes(
        now.signed_duration_since(member.user.id.created_at())
            .num_minutes(),
    );

    // Created at on second line
    writeln!(desc)?;
    // If less than 1 day old, give warning
    if age < Duration::days(1) {
        write!(desc, ":warning: New account: ",)?;
    }

    write!(
        desc,
        "Created at {}",
        humantime::format_duration(age.to_std().unwrap())
    )?;

    let res = member_log_channel
        .send_message(ctx, |m| {
            m.embed(|e| {
                e.author(|a| {
                    a.icon_url(member.user.face());
                    a.name(format!("{} (ID: {})", member.user.tag(), member.user.id));

                    a
                });

                e.description(desc);
                e.color(0x2ecc71);

                e
            })
        })
        .await;

    if let Err(SerenityError::Http(e)) = res {
        // Box cant be matched
        if let HttpError::UnsuccessfulRequest(e) = *e {
            // Unknown channel -- deleted channel so just unset
            if e.error.code == 10003 {
                guild_conf.log_member = None;
                guild_conf.save(ctx).await?;
            }

            // Missing access -- no perms so might as well just disable
            if e.error.code == 50001 {
                guild_conf.log_member_enabled = false;
                guild_conf.save(ctx).await?;
            }
        }
    }

    Ok(())
}

pub async fn guild_member_removal(
    ctx: &Context,
    guild_id: &GuildId,
    user: &User,
    member: &Option<Member>,
) {
    if let Err(e) = _guild_member_removal(ctx, guild_id, user, member).await {
        tracing::error!("Failed to handle guild_member_removal: {}", e);
    }
}

#[tracing::instrument(skip(ctx))]
async fn _guild_member_removal(
    ctx: &Context,
    guild_id: &GuildId,
    user: &User,
    member: &Option<Member>,
) -> Result<()> {
    let mut guild_conf = match GuildConfig::from_id(&ctx, &guild_id).await? {
        Some(c) => c,
        None => {
            tracing::error!(
                ?guild_id,
                "No guild config found while handling mute guild_member_removal"
            );
            return Ok(());
        }
    };

    if !guild_conf.log_member_enabled {
        return Ok(());
    }

    let member_log_channel = match guild_conf.log_member {
        Some(id) => ChannelId(id as u64),
        None => return Ok(()),
    };

    let mut desc = String::new();

    write!(desc, "{} left", user.mention())?;

    if let Some(member) = member {
        if let Some(joined_at) = member.joined_at {
            let now = Utc::now();
            // Truncate to seconds
            let joined_duration =
                Duration::seconds(now.signed_duration_since(joined_at).num_seconds());

            writeln!(desc)?;
            write!(
                desc,
                "Joined {} ago",
                humantime::format_duration(joined_duration.to_std().unwrap())
            )?;
        }
    }

    let res = member_log_channel
        .send_message(ctx, |m| {
            m.embed(|e| {
                e.author(|a| {
                    a.icon_url(user.face());
                    a.name(format!("{} (ID: {})", user.tag(), user.id));

                    a
                });

                e.description(desc);
                e.color(0xe74c3c);

                e
            })
        })
        .await;

    if let Err(SerenityError::Http(e)) = res {
        // Box cant be matched
        if let HttpError::UnsuccessfulRequest(e) = *e {
            // Unknown channel -- deleted channel so just unset
            if e.error.code == 10003 {
                guild_conf.log_member = None;
                guild_conf.save(ctx).await?;
            }

            // Missing access -- no perms so might as well just disable
            if e.error.code == 50001 {
                guild_conf.log_member_enabled = false;
                guild_conf.save(ctx).await?;
            }
        }
    }

    Ok(())
}
