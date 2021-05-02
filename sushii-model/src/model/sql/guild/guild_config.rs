use serde::{Deserialize, Serialize};
use serenity::model::prelude::*;
use serenity::prelude::*;
use serenity::utils::parse_channel;
use std::convert::TryFrom;
use std::fmt;
use std::time::Duration;

use crate::model::sql::GuildSetting;
use crate::prelude::*;

#[derive(Deserialize, Default, Serialize, sqlx::FromRow, Clone, Debug)]
pub struct GuildConfig {
    pub id: i64,
    pub prefix: Option<String>,

    /// Join message text
    pub join_msg: Option<String>,
    pub join_msg_enabled: bool,

    /// Join message reaction
    pub join_react: Option<String>,

    /// Leave message text
    pub leave_msg: Option<String>,
    pub leave_msg_enabled: bool,

    /// Join / leave messages channel
    pub msg_channel: Option<i64>,

    /// Role assignments
    pub role_channel: Option<i64>,
    pub role_config: Option<serde_json::Value>,
    pub role_enabled: bool,

    /// Auto delete invite links
    pub invite_guard: bool,

    /// Message deleted / edited log channel
    pub log_msg: Option<i64>,
    pub log_msg_enabled: bool,

    /// Moderation actions log channel
    pub log_mod: Option<i64>,
    pub log_mod_enabled: bool,

    /// Member join / leave log channel
    pub log_member: Option<i64>,
    pub log_member_enabled: bool,

    /// Mute role ID
    pub mute_role: Option<i64>,
    /// Duration in seconds
    pub mute_duration: Option<i64>,

    /// Should DM user on warn
    pub warn_dm_text: Option<String>,
    pub warn_dm_enabled: bool,

    /// Should DM user on mute
    pub mute_dm_text: Option<String>,
    pub mute_dm_enabled: bool,

    /// Max number of unique mentions in a single message to auto mute
    pub max_mention: Option<i32>,

    /// Channels where commands are ignored
    pub disabled_channels: Option<Vec<i64>>,
}

impl GuildConfig {
    pub fn new(id: i64) -> Self {
        GuildConfig {
            id,
            // Toggles should default to true since it still requires the main
            // setting to be set, if setting is set then it should mean they want
            // it to be turned on then.
            join_msg_enabled: true,
            leave_msg_enabled: true,
            log_msg_enabled: true,
            log_mod_enabled: true,
            log_member_enabled: true,
            mute_dm_enabled: true,
            warn_dm_enabled: true,
            ..Default::default()
        }
    }

    pub fn set_setting(&mut self, setting: &GuildSetting, val: &str) -> Result<()> {
        match setting {
            GuildSetting::JoinMsg => {
                self.join_msg.replace(val.into());
            }
            GuildSetting::JoinReact => {
                self.join_react.replace(
                    // Convert to ReactionType to validate, then convert back to string to save it
                    ReactionType::try_from(val)
                        .map_err(|_| Error::Sushii("invalid emoji".into()))?
                        .to_string(),
                );
            }
            GuildSetting::LeaveMsg => {
                self.leave_msg.replace(val.into());
            }
            GuildSetting::MsgChannel => {
                self.msg_channel.replace(
                    parse_channel(val).ok_or_else(|| Error::Sushii("invalid channel".into()))?
                        as i64,
                );
            }
            GuildSetting::MsgLog => {
                self.log_msg.replace(
                    parse_channel(val).ok_or_else(|| Error::Sushii("invalid channel".into()))?
                        as i64,
                );
            }
            GuildSetting::ModLog => {
                self.log_mod.replace(
                    parse_channel(val).ok_or_else(|| Error::Sushii("invalid channel".into()))?
                        as i64,
                );
            }
            GuildSetting::MemberLog => {
                self.log_member.replace(
                    parse_channel(val).ok_or_else(|| Error::Sushii("invalid channel".into()))?
                        as i64,
                );
            }
            GuildSetting::MuteDm => {
                self.mute_dm_text.replace(val.into());
            }
            GuildSetting::WarnDm => {
                self.mute_dm_text.replace(val.into());
            }
        }

        Ok(())
    }

    /// Enables/Disables a given setting, returns Ok(true) if successfully changed,
    /// Ok(false) if it is already enabled and Err() if the setting cannot be
    /// enabled or disabled
    fn update_setting(&mut self, setting: &GuildSetting, new_value: bool) -> Result<bool> {
        match setting {
            GuildSetting::JoinMsg => {
                if self.join_msg_enabled == new_value {
                    return Ok(false);
                }

                self.join_msg_enabled = new_value;
            }
            GuildSetting::LeaveMsg => {
                if self.leave_msg_enabled == new_value {
                    return Ok(false);
                }

                self.leave_msg_enabled = new_value;
            }
            GuildSetting::MsgLog => {
                if self.log_msg_enabled == new_value {
                    return Ok(false);
                }

                self.log_msg_enabled = new_value;
            }
            GuildSetting::ModLog => {
                if self.log_mod_enabled == new_value {
                    return Ok(false);
                }

                self.log_mod_enabled = new_value;
            }
            GuildSetting::MemberLog => {
                if self.log_member_enabled == new_value {
                    return Ok(false);
                }

                self.log_member_enabled = new_value;
            }
            GuildSetting::MuteDm => {
                if self.mute_dm_enabled == new_value {
                    return Ok(false);
                }

                self.mute_dm_enabled = new_value;
            }
            GuildSetting::WarnDm => {
                if self.warn_dm_enabled == new_value {
                    return Ok(false);
                }

                self.warn_dm_enabled = new_value;
            }
            GuildSetting::JoinReact | GuildSetting::MsgChannel => {
                return Err(Error::Sushii(
                    "this setting cannot be enabled/disabled".into(),
                ));
            }
        }

        Ok(true)
    }

    pub fn enable_setting(&mut self, setting: &GuildSetting) -> Result<bool> {
        self.update_setting(setting, true)
    }

    pub fn disable_setting(&mut self, setting: &GuildSetting) -> Result<bool> {
        self.update_setting(setting, false)
    }

    /// Toggles a setting and returns it's new value
    pub fn toggle_setting(&mut self, setting: &GuildSetting) -> Result<bool> {
        let new_val = match setting {
            GuildSetting::JoinMsg => {
                self.join_msg_enabled = !self.join_msg_enabled;
                self.join_msg_enabled
            }
            GuildSetting::LeaveMsg => {
                self.leave_msg_enabled = !self.leave_msg_enabled;
                self.leave_msg_enabled
            }
            GuildSetting::MsgLog => {
                self.log_msg_enabled = !self.log_msg_enabled;
                self.log_msg_enabled
            }
            GuildSetting::ModLog => {
                self.log_mod_enabled = !self.log_mod_enabled;
                self.log_mod_enabled
            }
            GuildSetting::MemberLog => {
                self.log_member_enabled = !self.log_member_enabled;
                self.log_member_enabled
            }
            GuildSetting::MuteDm => {
                self.mute_dm_enabled = !self.mute_dm_enabled;
                self.mute_dm_enabled
            }
            GuildSetting::WarnDm => {
                self.warn_dm_enabled = !self.warn_dm_enabled;
                self.warn_dm_enabled
            }
            GuildSetting::JoinReact | GuildSetting::MsgChannel => {
                return Err(Error::Sushii(
                    "this setting cannot be enabled/disabled".into(),
                ));
            }
        };

        Ok(new_val)
    }

    pub fn get_setting(&self, setting: &GuildSetting) -> (Option<String>, Option<bool>) {
        match setting {
            GuildSetting::JoinMsg => (self.join_msg.clone(), Some(self.join_msg_enabled)),
            GuildSetting::LeaveMsg => (self.leave_msg.clone(), Some(self.leave_msg_enabled)),
            GuildSetting::MsgLog => (
                self.log_msg.map(|id| format!("<#{}>", id as u64)),
                Some(self.log_msg_enabled),
            ),
            GuildSetting::ModLog => (
                self.log_mod.map(|id| format!("<#{}>", id as u64)),
                Some(self.log_mod_enabled),
            ),
            GuildSetting::MuteDm => (self.mute_dm_text.clone(), Some(self.mute_dm_enabled)),
            GuildSetting::WarnDm => (self.warn_dm_text.clone(), Some(self.warn_dm_enabled)),
            GuildSetting::JoinReact => (self.join_react.clone(), None),
            GuildSetting::MsgChannel => {
                (self.msg_channel.map(|id| format!("<#{}>", id as u64)), None)
            }
            GuildSetting::MemberLog => {
                (self.log_member.map(|id| format!("<#{}>", id as u64)), None)
            }
        }
    }

    /// Gets a GuildConfig from a given message
    pub async fn from_msg(ctx: &Context, msg: &Message) -> Result<Option<GuildConfig>> {
        GuildConfig::get(ctx, Some(msg), None).await
    }

    /// Gets a GuildConfig from a message, and responds in channel if no guild is found
    pub async fn from_msg_or_respond(ctx: &Context, msg: &Message) -> Result<GuildConfig> {
        match GuildConfig::from_msg(ctx, msg).await? {
            Some(conf) => Ok(conf),
            None => {
                if let Err(e) = msg
                    .channel_id
                    .say(&ctx.http, "Failed to get the guild config :(")
                    .await
                {
                    tracing::error!(?msg, "Failed to send message: {}", e);
                }

                tracing::warn!(?msg, "Failed to get guild config");

                Err(Error::Sushii("Failed to get guild config".into()))
            }
        }
    }

    /// Gets from db without cache
    pub async fn from_id_db(pool: &sqlx::PgPool, guild_id: u64) -> Result<Option<GuildConfig>> {
        get_guild_config_query(pool, guild_id).await
    }

    /// Gets a Guildconfig from a guild id
    pub async fn from_id(ctx: &Context, guild_id: &GuildId) -> Result<Option<GuildConfig>> {
        GuildConfig::get(ctx, None, Some(guild_id)).await
    }

    /// Gets a GuildConfig from the cache or database
    pub async fn get(
        ctx: &Context,
        msg: Option<&Message>,
        guild_id: Option<&GuildId>,
    ) -> Result<Option<GuildConfig>> {
        // Return None if no guild found
        let guild_id = match guild_id.or_else(|| msg.and_then(|m| m.guild_id.as_ref())) {
            Some(id) => id,
            None => return Ok(None),
        };

        let sushii_cache = ctx.data.read().await.get::<SushiiCache>().cloned().unwrap();

        // Check if exists in cache
        if sushii_cache.guilds.contains_key(&guild_id) {
            return Ok(sushii_cache
                .guilds
                .get(&guild_id)
                .map(|e| e.value().clone()));
        }

        // Not in cache, fetch from database
        let pool = ctx.data.read().await.get::<DbPool>().cloned().unwrap();
        let conf = match get_guild_config_query(&pool, guild_id.0)
            .await
            .map_err(|e| {
                tracing::error!(
                    ?msg,
                    ?guild_id,
                    "Failed to get guild config from database: {}",
                    e
                );

                e
            })? {
            Some(c) => c,
            None => {
                let new_conf = GuildConfig::new(guild_id.0 as i64);

                if let Err(e) = new_conf.save_db(&ctx).await {
                    tracing::error!("Failed to save new guild config: {}", e);
                }

                new_conf
            }
        };

        conf.cache(&ctx).await;

        Ok(Some(conf))
    }

    /// Updates config in the cache
    pub async fn cache(&self, ctx: &Context) {
        let sushii_cache = ctx.data.read().await.get::<SushiiCache>().cloned().unwrap();

        sushii_cache
            .guilds
            .insert(GuildId(self.id as u64), self.clone());

        tracing::info!(config = ?self, "Cached guild config");
    }

    /// Saves config to database
    pub async fn save_db(&self, ctx: &Context) -> Result<()> {
        let pool = ctx.data.read().await.get::<DbPool>().cloned().unwrap();

        // Update db and cache
        upsert_config_query(self, &pool).await?;
        Ok(())
    }

    /// Saves config to both database and cache
    pub async fn save(&self, ctx: &Context) -> Result<()> {
        self.save_db(&ctx).await?;
        self.cache(&ctx).await;

        Ok(())
    }
}

fn fmt_channel(id: Option<i64>) -> Option<String> {
    id.map(|id| format!("<#{}>", id))
}

fn fmt_role(id: Option<i64>) -> Option<String> {
    id.map(|id| format!("<@&{}>", id))
}

fn fmt_num(num: Option<i32>) -> Option<String> {
    num.map(|num| num.to_string())
}

fn fmt_duration(d: Option<i64>) -> Option<String> {
    d.map(|d| d as u64)
        .map(Duration::from_secs)
        .map(|d| humantime::format_duration(d).to_string())
}

impl fmt::Display for GuildConfig {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // (Name, Value, Option<Enabled>)
        let fields = [
            ("Prefix", Some(self.prefix.clone()), None),
            (
                "Join Message",
                Some(self.join_msg.clone()),
                Some(self.join_msg_enabled),
            ),
            ("Join React", Some(self.join_react.clone()), None),
            (
                "Leave Message",
                Some(self.leave_msg.clone()),
                Some(self.leave_msg_enabled),
            ),
            ("Message Channel", Some(fmt_channel(self.msg_channel)), None),
            (
                "Message Log",
                Some(fmt_channel(self.log_msg)),
                Some(self.log_msg_enabled),
            ),
            (
                "Mod Log",
                Some(fmt_channel(self.log_mod)),
                Some(self.log_mod_enabled),
            ),
            (
                "Member Log",
                Some(fmt_channel(self.log_member)),
                Some(self.log_member_enabled),
            ),
            ("Mute Role", Some(fmt_role(self.mute_role)), None),
            (
                "Mute Default Duration",
                Some(fmt_duration(self.mute_duration)),
                None,
            ),
            (
                "Warn DM",
                Some(self.warn_dm_text.clone()),
                Some(self.warn_dm_enabled),
            ),
            (
                "Mute DM",
                Some(self.mute_dm_text.clone()),
                Some(self.mute_dm_enabled),
            ),
            (
                "Roles Channel",
                Some(fmt_channel(self.role_channel)),
                Some(self.role_enabled),
            ),
            ("Invite Guard", None, Some(self.invite_guard)),
            ("Max Mentions", Some(fmt_num(self.max_mention)), None),
            // role_config: Option<serde_json::Value>,
        ];

        for field in fields.iter() {
            if let Some(enabled) = field.2 {
                if enabled {
                    write!(f, "<:online:316354435745972244>")?;
                } else {
                    write!(f, "<:offline:316354467031416832>")?;
                }
            }

            if let Some(value) = field.1.as_ref() {
                write!(f, " **{}:** ", field.0)?;

                match value {
                    Some(v) => write!(f, " {}", v)?,
                    None => write!(f, r#" \_\_\_\_\_\_"#)?,
                };
            } else {
                // Fields that don't have a value shouldn't have the ":"
                write!(f, " **{}** ", field.0)?;
            }

            writeln!(f)?;
        }

        Ok(())
    }
}

async fn get_guild_config_query(pool: &sqlx::PgPool, guild_id: u64) -> Result<Option<GuildConfig>> {
    sqlx::query_as!(
        GuildConfig,
        r#"
            SELECT *
              FROM app_public.guild_configs
             WHERE id = $1
        "#,
        guild_id as i64
    )
    .fetch_optional(pool)
    .await
    .map_err(Into::into)
}

async fn upsert_config_query(conf: &GuildConfig, pool: &sqlx::PgPool) -> Result<()> {
    // Bruh
    sqlx::query_file!(
        "sql/guild/upsert_guild_config.sql",
        conf.id,
        conf.prefix,
        conf.join_msg,
        conf.join_msg_enabled,
        conf.join_react,
        conf.leave_msg,
        conf.leave_msg_enabled,
        conf.msg_channel,
        conf.role_channel,
        conf.role_config,
        conf.role_enabled,
        conf.invite_guard,
        conf.log_msg,
        conf.log_msg_enabled,
        conf.log_mod,
        conf.log_mod_enabled,
        conf.log_member,
        conf.log_member_enabled,
        conf.mute_role,
        conf.mute_duration,
        conf.warn_dm_text,
        conf.warn_dm_enabled,
        conf.mute_dm_text,
        conf.mute_dm_enabled,
        conf.max_mention,
        conf.disabled_channels.as_deref(),
    )
    .execute(pool)
    .await
    .map(|_| ())
    .map_err(Into::into)
}
