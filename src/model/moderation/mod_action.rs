use lazy_static::lazy_static;
use regex::Regex;
use serenity::async_trait;
use serenity::framework::standard::Args;
use serenity::model::prelude::*;
use serenity::prelude::*;
use serenity::CacheAndHttp;
use serenity::Error;
use std::collections::HashSet;
use std::fmt;
use std::fmt::Write;
use std::result::Result as StdResult;
use std::sync::Arc;

use crate::error::{Error as SushiiError, Result};
use crate::keys::CacheAndHttpContainer;
use crate::model::sql::{GuildConfig, GuildConfigDb, ModLogEntry, ModLogEntryDb};

#[derive(Debug)]
pub enum ModActionType {
    Ban,
    Unban,
    Kick,
    Mute,
    Unmute,
}

impl fmt::Display for ModActionType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                ModActionType::Ban => "ban",
                ModActionType::Unban => "unban",
                ModActionType::Kick => "kick",
                ModActionType::Mute => "mute",
                ModActionType::Unmute => "unmute",
            }
        )
    }
}

impl ModActionType {
    pub fn to_past_tense(&self) -> String {
        match self {
            ModActionType::Ban => "banned",
            ModActionType::Unban => "unbanned",
            ModActionType::Kick => "kicked",
            ModActionType::Mute => "muted",
            ModActionType::Unmute => "unmuted",
        }
        .into()
    }

    pub fn to_emoji(&self) -> String {
        match self {
            ModActionType::Ban => ":hammer:",
            ModActionType::Unban => ":hammer:",
            ModActionType::Kick => ":boot:",
            ModActionType::Mute => ":mute:",
            ModActionType::Unmute => ":speaker:",
        }
        .into()
    }
}

#[async_trait]
pub trait ModActionExecutorDb {
    async fn execute_user(
        &self,
        ctx: &Context,
        cache_http: &Arc<CacheAndHttp>,
        user: &User,
        guild_id: &GuildId,
        guild_conf: &GuildConfig,
    ) -> StdResult<(), Error>;
    async fn execute(mut self, ctx: &Context, msg: &Message, guild_id: &GuildId) -> Result<()>;
}

#[derive(Debug)]
pub struct ModActionExecutor {
    pub action: ModActionType,
    pub target_users: Vec<u64>,
    pub exclude_users: HashSet<u64>,
    pub reason: Option<String>,
}

impl ModActionExecutor {
    pub fn from_args(args: Args, action: ModActionType) -> Self {
        let (target_users, reason) = parse_id_reason(args);

        Self {
            action,
            target_users,
            exclude_users: HashSet::new(),
            reason,
        }
    }

    pub fn exclude_users<I: IntoIterator<Item = u64>>(mut self, exclude_users: I) -> Self {
        exclude_users.into_iter().for_each(|id| {
            self.exclude_users.insert(id);
        });
        self
    }
}

#[async_trait]
impl ModActionExecutorDb for ModActionExecutor {
    // Well... kind of yeah
    async fn execute_user(
        &self,
        ctx: &Context,
        cache_http: &Arc<CacheAndHttp>,
        user: &User,
        guild_id: &GuildId,
        guild_conf: &GuildConfig,
    ) -> StdResult<(), Error> {
        match self.action {
            ModActionType::Ban => {
                if let Some(reason) = &self.reason {
                    guild_id
                        .ban_with_reason(&ctx.http, user, 7u8, &reason)
                        .await?;
                } else {
                    guild_id.ban(&ctx.http, user, 7u8).await?;
                }
            }
            ModActionType::Unban => {
                guild_id.unban(&ctx.http, user).await?;
            }
            ModActionType::Kick => {
                if let Some(reason) = &self.reason {
                    guild_id.kick_with_reason(&ctx.http, user, &reason).await?;
                } else {
                    guild_id.kick(&ctx.http, user).await?;
                }
            }
            ModActionType::Mute => {
                // Mute commands should check if mute role exists before running ::execute()
                if let Some(role_id) = guild_conf.mute_role {
                    let mut member = guild_id.member(&cache_http, user).await?;

                    member.add_role(&ctx.http, role_id as u64).await?;
                }
            }
            ModActionType::Unmute => {
                if let Some(role_id) = guild_conf.mute_role {
                    let mut member = guild_id.member(&cache_http, user).await?;

                    member.remove_role(&ctx.http, role_id as u64).await?;
                }
            }
        }

        Ok(())
    }

    async fn execute(mut self, ctx: &Context, msg: &Message, guild_id: &GuildId) -> Result<()> {
        let data = &ctx.data.read().await;
        let cache_http = data.get::<CacheAndHttpContainer>().unwrap();

        let guild_conf = GuildConfig::from_id(&ctx, guild_id)
            .await?
            .ok_or_else(|| SushiiError::Sushii("No guild found".into()))?;

        let action_str = self.action.to_string();
        let action_past_str = self.action.to_past_tense();

        let mut s = String::new();

        for &id in &self.target_users {
            let user = match UserId(id).to_user(cache_http).await {
                Ok(u) => u,
                Err(e) => {
                    let _ = writeln!(s, ":x: {} - Error: Failed to fetch user: {}", id, &e);

                    continue;
                }
            };

            let user_tag_id = format!("`{} ({})`", user.tag(), user.id.0);

            if self.exclude_users.contains(&id) {
                let _ = writeln!(
                    s,
                    ":x: {} - Error: User is already {}",
                    user_tag_id, &action_past_str
                );
                continue;
            }

            let entry = match ModLogEntry::new(&self.action.to_string(), true, guild_id.0, &user)
                .reason(&self.reason)
                .executor_id(msg.author.id.0)
                .save(&ctx)
                .await
            {
                Ok(v) => v,
                Err(e) => {
                    tracing::error!("Failed to save mod log entry: {}", e);

                    let _ = writeln!(
                        s,
                        ":x: {} - Error: Something went wrong saving this case :(",
                        &user_tag_id
                    );
                    continue;
                }
            };

            let res = self
                .execute_user(&ctx, &cache_http, &user, &guild_id, &guild_conf)
                .await;

            match res {
                Err(Error::Model(ModelError::InvalidPermissions(permissions))) => {
                    let _ = writeln!(s, ":question: {} - Error: I don't have permission to {} this user, requires: `{:?}`.", &user_tag_id, &action_str, permissions);
                    if let Err(e) = entry.delete(&ctx).await {
                        tracing::error!("Failed to delete entry: {}", e);
                    }
                }
                Err(Error::Model(ModelError::DeleteMessageDaysAmount(num))) => {
                    let _ = writeln!(s, ":x: {} - Error: The number of days worth of messages to delete is over the maximum: ({}).", &user_tag_id, &num);
                    if let Err(e) = entry.delete(&ctx).await {
                        tracing::error!("Failed to delete entry: {}", e);
                    }
                }
                Err(e) => {
                    let _ = writeln!(s, ":question: {} - Error: {}", &user_tag_id, &e);
                    if let Err(e) = entry.delete(&ctx).await {
                        tracing::error!("Failed to delete entry: {}", e);
                    }
                }
                Ok(_) => {
                    let _ = writeln!(
                        s,
                        "{} {} {}.",
                        self.action.to_emoji(),
                        &user_tag_id,
                        &action_past_str
                    );
                    // add the action to hashset to prevent dupe actions
                    self.exclude_users.insert(id);
                }
            }
        }

        // Respond to user
        let _ = msg.channel_id.say(&ctx.http, &s).await?;

        Ok(())
    }
}

pub fn parse_id_reason(args: Args) -> (Vec<u64>, Option<String>) {
    lazy_static! {
        // Can overflow, so need to handle later
        static ref RE: Regex = Regex::new(r"(?:<@)?(\d{18,19})>?").unwrap();
    }

    let ids_and_reason = args.rest();

    let (ids, end) = RE
        .captures_iter(ids_and_reason)
        .fold((Vec::new(), 0), |mut acc, caps| {
            if let Some(id) = caps.get(1).and_then(|m| m.as_str().parse::<u64>().ok()) {
                acc.0.push(id);
                // First capture group is entire match so it must exist
                acc.1 = caps.get(0).unwrap().end();
            }

            acc
        });

    let reason = {
        let r = ids_and_reason[end..].trim().to_string();

        if r.is_empty() {
            None
        } else {
            Some(r)
        }
    };

    (ids, reason)
}

#[test]
fn parses_ids_and_reason() {
    use serenity::framework::standard::Delimiter;

    let ids_exp = vec![145764790046818304, 193163974471188480, 151018674793349121];
    let reason_exp = Some("some reason text".to_string());

    let input_strs = vec![
        // Comma separated
        "145764790046818304,193163974471188480,151018674793349121 some reason text",
        // Mentions
        "<@145764790046818304> <@193163974471188480> <@151018674793349121> some reason text",
        // Space separated
        "145764790046818304 193163974471188480 151018674793349121 some reason text",
        // Random chars in middle
        "145764790046818304   193163974471188480 aoweifjf 151018674793349121 some reason text",
    ];

    for s in input_strs {
        let args = Args::new(s, &[Delimiter::Single(' ')]);

        let (ids, reason) = parse_id_reason(args);

        assert_eq!(ids, ids_exp);
        assert_eq!(reason, reason_exp);
    }
}

#[test]
fn parses_ids_without_reason() {
    use serenity::framework::standard::Delimiter;

    let ids_exp = vec![145764790046818304, 193163974471188480, 151018674793349121];

    let input_strs = vec![
        // Comma separated
        "145764790046818304,193163974471188480,151018674793349121",
        // Mentions
        "<@145764790046818304> <@193163974471188480> <@151018674793349121>",
        // Space separated
        "145764790046818304 193163974471188480 151018674793349121 ",
        // Random chars in middle
        "145764790046818304   193163974471188480 aoweifjf 151018674793349121              ",
    ];

    for s in input_strs {
        let args = Args::new(s, &[Delimiter::Single(' ')]);

        let (ids, reason) = parse_id_reason(args);

        assert_eq!(ids, ids_exp);
        assert!(reason.is_none());
    }
}
