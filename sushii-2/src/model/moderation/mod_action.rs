// use chrono::Duration;
// use lazy_static::lazy_static;
// use regex::Regex;
// use serenity::framework::standard::Args;
// use serenity::model::prelude::*;
// use serenity::prelude::*;
// use serenity::Error;
// use std::collections::HashSet;
// use std::fmt;
// use std::fmt::Write;
// use std::result::Result as StdResult;

// use crate::error::{Error as SushiiError, Result};
// use crate::model::moderation::ModLogReporter;
// use crate::model::sql::{GuildConfig, ModLogEntry, Mute};
// use sushii_model::utils::duration::{find_duration, parse_duration};

// #[derive(Debug, PartialEq, Eq)]
// pub enum ModActionType {
//     Ban,
//     Unban,
//     Kick,
//     Mute,
//     Unmute,
//     Warn,
//     Note,
// }

// impl fmt::Display for ModActionType {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         write!(
//             f,
//             "{}",
//             match self {
//                 ModActionType::Ban => "ban",
//                 ModActionType::Unban => "unban",
//                 ModActionType::Kick => "kick",
//                 ModActionType::Mute => "mute",
//                 ModActionType::Unmute => "unmute",
//                 ModActionType::Warn => "warn",
//                 ModActionType::Note => "note",
//             }
//         )
//     }
// }

// impl ModActionType {
//     pub fn to_present_tense(&self) -> &'static str {
//         match self {
//             ModActionType::Ban => "ban",
//             ModActionType::Unban => "unban",
//             ModActionType::Kick => "kick",
//             ModActionType::Mute => "mute",
//             ModActionType::Unmute => "unmute",
//             ModActionType::Warn => "warn",
//             ModActionType::Note => "add note to",
//         }
//     }

//     pub fn to_past_tense(&self) -> &'static str {
//         match self {
//             ModActionType::Ban => "banned",
//             ModActionType::Unban => "unbanned",
//             ModActionType::Kick => "kicked",
//             ModActionType::Mute => "muted",
//             ModActionType::Unmute => "unmuted",
//             ModActionType::Warn => "warned",
//             ModActionType::Note => "note added",
//         }
//     }

//     pub fn to_emoji(&self) -> &'static str {
//         match self {
//             ModActionType::Ban => ":hammer:",
//             ModActionType::Unban => ":hammer:",
//             ModActionType::Kick => ":boot:",
//             ModActionType::Mute => ":mute:",
//             ModActionType::Unmute => ":speaker:",
//             ModActionType::Warn => ":warning:",
//             ModActionType::Note => ":pencil:",
//         }
//     }
// }

// #[derive(Debug)]
// pub struct ModActionExecutor {
//     pub action: ModActionType,
//     pub target_users: Vec<u64>,
//     pub exclude_users: HashSet<u64>,
//     pub reason: Option<String>,

//     /// Duration is only for mutes
//     pub duration: Option<StdResult<Duration, String>>,
// }

// impl ModActionExecutor {
//     pub fn from_args(args: Args, action: ModActionType) -> Self {
//         let (target_users, reason, duration) = if action == ModActionType::Mute {
//             parse_id_reason_duration(args)
//         } else {
//             let (target_users, reason) = parse_id_reason(args);

//             (target_users, reason, None)
//         };

//         Self {
//             action,
//             target_users,
//             exclude_users: HashSet::new(),
//             reason,
//             duration,
//         }
//     }

//     pub fn exclude_users<I: IntoIterator<Item = u64>>(mut self, exclude_users: I) -> Self {
//         exclude_users.into_iter().for_each(|id| {
//             self.exclude_users.insert(id);
//         });
//         self
//     }

//     #[allow(clippy::too_many_arguments)]
//     async fn execute_user(
//         &self,
//         ctx: &Context,
//         msg: &Message,
//         user: &User,
//         guild: &Option<Guild>,
//         guild_id: &GuildId,
//         guild_conf: &GuildConfig,
//         duration: &Option<Duration>,
//     ) -> Result<Option<String>> {
//         match self.action {
//             ModActionType::Ban => {
//                 if let Some(reason) = &self.reason {
//                     guild_id
//                         .ban_with_reason(
//                             &ctx.http,
//                             user,
//                             0u8,
//                             format!(
//                                 "[Ban by {} (ID: {})] {}",
//                                 &msg.author.tag(),
//                                 &msg.author.id.0,
//                                 &reason
//                             ),
//                         )
//                         .await?;
//                 } else {
//                     guild_id
//                         .ban_with_reason(
//                             &ctx.http,
//                             user,
//                             0u8,
//                             format!(
//                                 "[Ban by {} (ID: {})] No reason provided",
//                                 &msg.author.tag(),
//                                 &msg.author.id.0,
//                             ),
//                         )
//                         .await?;
//                 }
//             }
//             ModActionType::Unban => {
//                 guild_id.unban(&ctx.http, user).await?;
//             }
//             ModActionType::Kick => {
//                 if let Some(reason) = &self.reason {
//                     guild_id
//                         .kick_with_reason(
//                             &ctx.http,
//                             user,
//                             &format!(
//                                 "[Kick by {} (ID: {})] {}",
//                                 &msg.author.tag(),
//                                 &msg.author.id.0,
//                                 &reason
//                             ),
//                         )
//                         .await?;
//                 } else {
//                     guild_id
//                         .kick_with_reason(
//                             &ctx.http,
//                             user,
//                             &format!(
//                                 "[Kick by {} (ID: {})] No reason provided",
//                                 &msg.author.tag(),
//                                 &msg.author.id.0,
//                             ),
//                         )
//                         .await?;
//                 }
//             }
//             ModActionType::Mute => {
//                 // Mute commands should check if mute role exists before running ::execute()
//                 if let Some(role_id) = guild_conf.mute_role {
//                     let mut member = guild_id.member(ctx, user).await?;

//                     // Handle if already muted, respond with error
//                     if member.roles.contains(&RoleId(role_id as u64)) {
//                         return Err(SushiiError::Sushii("User is already muted".into()));
//                     }

//                     // Add a pending mute entry
//                     Mute::new(guild_id.0, user.id.0, *duration)
//                         .pending(true)
//                         .save(&ctx)
//                         .await?;

//                     member.add_role(&ctx.http, role_id as u64).await?;
//                 }
//             }
//             ModActionType::Unmute => {
//                 if let Some(role_id) = guild_conf.mute_role {
//                     let mut member = guild_id.member(ctx, user).await?;

//                     member.remove_role(&ctx.http, role_id as u64).await?;
//                 }
//             }
//             ModActionType::Warn => {
//                 // Warn does nothing other than make a mod log entry
//                 // But since mod log messages are sent via event handlers like
//                 // guild_ban_addition, there isn't an event to handle for warns.
//                 // So we just send it here right after creating a pending entry,
//                 // created right before this execute_user() function is called
//                 ModLogReporter::new(guild_id, user, "warn")
//                     .execute(&ctx)
//                     .await?;

//                 if !guild_conf.warn_dm_enabled {
//                     return Ok(Some(" Warn DMs are not enabled, user not DMed.".into()));
//                 }

//                 let guild_name = guild
//                     .as_ref()
//                     .map(|g| g.name.clone())
//                     .unwrap_or_else(|| format!("Unknown Guild (ID: {})", guild_id.0));

//                 let dm_res = user
//                     .dm(ctx, |m| {
//                         m.content(format!(
//                             "You have been warned in {}\nReason: {}",
//                             guild_name,
//                             self.reason
//                                 .clone()
//                                 .unwrap_or_else(|| "No reason given".into())
//                         ))
//                     })
//                     .await;

//                 if dm_res.is_err() {
//                     // Space in front since its added on at end
//                     return Ok(Some(
//                         " Failed to send DM, they possibly have them disabled".into(),
//                     ));
//                 }
//             }
//             ModActionType::Note => {
//                 ModLogReporter::new(guild_id, user, "note")
//                     .execute(&ctx)
//                     .await?;
//             }
//         }

//         Ok(None)
//     }

//     pub async fn execute(mut self, ctx: &Context, msg: &Message, guild_id: &GuildId) -> Result<()> {
//         let guild_conf = GuildConfig::from_id(&ctx, guild_id)
//             .await?
//             .ok_or_else(|| SushiiError::Sushii("No guild found".into()))?;

//         let guild = guild_id.to_guild_cached(ctx);

//         let action_str = self.action.to_string();
//         let action_past_str = self.action.to_past_tense();

//         if self.target_users.is_empty() {
//             msg.channel_id
//                 .say(
//                     &ctx,
//                     "No target users were found, please give valid IDs or mentions",
//                 )
//                 .await?;

//             return Ok(());
//         }

//         if let Some(Err(e)) = &self.duration {
//             // If there is a duration, check if the duration parsing failed
//             msg.channel_id
//                 .say(&ctx, format!("Invalid duration, {}", e))
//                 .await?;

//             return Ok(());
//         }

//         // Uh... clone then flatten the Option<Result<Duration>> to just
//         // Option<Duration>, otherwise just use guild config default
//         // This is either the provided duration, the guild config default duration, or just None for indefinite
//         let duration = self
//             .duration
//             .clone()
//             .map(|d| d.ok())
//             .flatten();

//         let mut sent_msg = msg
//             .channel_id
//             .say(
//                 &ctx,
//                 format!(
//                     "Attempting to {} {} users...",
//                     self.action.to_present_tense(),
//                     &self.target_users.len()
//                 ),
//             )
//             .await?;

//         let mut s = String::new();

//         for &id in &self.target_users {
//             let user = match UserId(id).to_user(ctx).await {
//                 Ok(u) => u,
//                 Err(e) => {
//                     let _ = writeln!(s, ":x: {} - Error: Failed to fetch user: {}", id, &e);

//                     continue;
//                 }
//             };

//             let user_tag_id = format!("`{} ({})`", user.tag(), user.id.0);

//             if self.exclude_users.contains(&id) {
//                 let _ = writeln!(
//                     s,
//                     ":x: {} - Error: User is already {}",
//                     user_tag_id, &action_past_str
//                 );
//                 continue;
//             }

//             let entry = match ModLogEntry::new(
//                 &self.action.to_string(),
//                 true,
//                 guild_id.0,
//                 user.id.0,
//                 &user.tag(),
//             )
//             .reason(&self.reason)
//             .executor_id(msg.author.id.0)
//             .save(&ctx)
//             .await
//             {
//                 Ok(v) => v,
//                 Err(e) => {
//                     tracing::error!("Failed to save mod log entry: {}", e);

//                     let _ = writeln!(
//                         s,
//                         ":x: {} - Error: Something went wrong saving this case :(",
//                         &user_tag_id
//                     );
//                     continue;
//                 }
//             };

//             let res = self
//                 .execute_user(&ctx, &msg, &user, &guild, &guild_id, &guild_conf, &duration)
//                 .await;

//             match res {
//                 // Bruh it's getting spaghetti'd again
//                 Err(SushiiError::Serenity(Error::Model(ModelError::InvalidPermissions(
//                     permissions,
//                 )))) => {
//                     let _ = writeln!(s, ":question: {} - Error: I don't have permission to {} this user, requires: `{:?}`.", &user_tag_id, &action_str, permissions);
//                     if let Err(e) = entry.delete(&ctx).await {
//                         tracing::error!("Failed to delete entry: {}", e);
//                     }
//                 }
//                 Err(SushiiError::Serenity(Error::Model(ModelError::DeleteMessageDaysAmount(
//                     num,
//                 )))) => {
//                     let _ = writeln!(s, ":x: {} - Error: The number of days worth of messages to delete is over the maximum: ({}).", &user_tag_id, &num);
//                     if let Err(e) = entry.delete(&ctx).await {
//                         tracing::error!("Failed to delete entry: {}", e);
//                     }
//                 }
//                 Err(e) => {
//                     let _ = writeln!(s, ":question: {} - Error: {}", &user_tag_id, &e);
//                     if let Err(e) = entry.delete(&ctx).await {
//                         tracing::error!("Failed to delete entry: {}", e);
//                     }
//                 }
//                 Ok(extra_str) => {
//                     let _ = writeln!(
//                         s,
//                         "{} {} {}.{}",
//                         self.action.to_emoji(),
//                         &user_tag_id,
//                         &action_past_str,
//                         &extra_str.unwrap_or_else(|| "".into()),
//                     );
//                     // add the action to hashset to prevent dupe actions
//                     self.exclude_users.insert(id);
//                 }
//             }
//         }

//         // Respond to user -- edit previously sent message
//         sent_msg
//             .edit(ctx, |m| {
//                 // Remove attempting to .. message
//                 m.content("");
//                 m.embed(|e| {
//                     e.title(format!(
//                         "Attempted to {} {} users",
//                         self.action.to_present_tense(),
//                         &self.target_users.len()
//                     ));
//                     e.description(&s);

//                     e.field(
//                         "Reason",
//                         self.reason.unwrap_or_else(|| "No reason given".into()),
//                         false,
//                     );

//                     if self.action == ModActionType::Mute {
//                         e.field(
//                             "Mute Duration",
//                             duration.map_or_else(
//                                 || "Indefinite".to_string(),
//                                 |d| humantime::format_duration(d.to_std().unwrap()).to_string(),
//                             ),
//                             false,
//                         );
//                     }

//                     e
//                 })
//             })
//             .await?;

//         Ok(())
//     }
// }

// fn parse_id_reason_duration(
//     args: Args,
// ) -> (
//     Vec<u64>,
//     Option<String>,
//     Option<StdResult<Duration, String>>,
// ) {
//     let (ids, reason) = parse_id_reason(args);

//     // Look for position of duration string
//     let duration_match = reason.as_ref().and_then(|s| find_duration(&s));

//     // Reason without the duration string
//     let reason_no_duration = duration_match.and_then(|d| {
//         reason
//             .clone()
//             // Remove duration string
//             .map(|r| r.replace(d.as_str(), "").trim().to_string())
//             // If resulting reason without duration is empty, then None for reason
//             .and_then(|r| if r.is_empty() { None } else { Some(r) })
//     });

//     // Parsed duration
//     let duration = duration_match.map(|d| parse_duration(d.as_str()));

//     // Original reason if there isn't a duration found
//     let processed_reason = if duration_match.is_none() {
//         // If there isn't a duration, should use the original reason
//         reason
//     } else {
//         reason_no_duration
//     };

//     // Return the ids, reason with the duration string removed, parsed duration
//     (ids, processed_reason, duration)
// }

// fn parse_id_reason(args: Args) -> (Vec<u64>, Option<String>) {
//     lazy_static! {
//         // Can overflow, so need to handle later
//         // Excludes if ID haas # or & before the digits to exclude channel and role mentions
//         // Allows ^ start of string for raw ID or <@ for user mention
//         static ref RE: Regex = Regex::new(r"(?:<@|[^#&\d]|^)(\d{17,19})>?").unwrap();
//     }

//     let ids_and_reason = args.rest();

//     // If there's a reason (alphabetic chars, use that as the end) in case there
//     // are IDs or mentions in the reason which we don't want to include in the
//     // actual action
//     let reason_start = ids_and_reason.find(char::is_alphabetic);
//     let ids_substr = if let Some(index) = reason_start {
//         &ids_and_reason[..index]
//     } else {
//         &ids_and_reason[..]
//     };

//     let (mut ids, end) =
//         RE.captures_iter(ids_substr)
//             .enumerate()
//             .fold((Vec::new(), 0), |mut acc, (i, caps)| {
//                 if let Some(id) = caps.get(1).and_then(|m| m.as_str().parse::<u64>().ok()) {
//                     acc.0.push((i, id));
//                     // First capture group is entire match so it must exist
//                     acc.1 = caps.get(0).unwrap().end();
//                 }

//                 acc
//             });

//     // First sort by ID
//     ids.sort_by(|a, b| a.1.cmp(&b.1));
//     // Dedupe by IDs
//     ids.dedup_by(|a, b| a.1.eq(&b.1));
//     // Sort by original order
//     ids.sort_by(|a, b| a.0.cmp(&b.0));

//     // Remove indexes
//     let ids = ids.into_iter().map(|id| id.1).collect();

//     let reason = {
//         let r = ids_and_reason[end..].trim().to_string();

//         if r.is_empty() {
//             None
//         } else {
//             Some(r)
//         }
//     };

//     (ids, reason)
// }

// #[cfg(test)]
// mod tests {
//     use super::*;
//     use serenity::framework::standard::Delimiter;
//     const IDS_EXP: &'static [u64] = &[145764790046818304, 193163974471188480, 151018674793349121];
//     const REASON_EXP: &str = "some reason text";

//     #[test]
//     fn parses_ids_and_reason() {
//         let input_strs = vec![
//             // Comma separated
//             "145764790046818304,193163974471188480,151018674793349121 some reason text",
//             // Mentions
//             "<@145764790046818304> <@193163974471188480> <@151018674793349121> some reason text",
//             // Space separated
//             "145764790046818304 193163974471188480 151018674793349121 some reason text",
//             // Random spacing
//             "145764790046818304   193163974471188480    151018674793349121 some reason text",
//         ];

//         for s in input_strs {
//             let args = Args::new(s, &[Delimiter::Single(' ')]);

//             let (ids, reason) = parse_id_reason(args);

//             assert_eq!(ids, IDS_EXP);
//             assert_eq!(reason.unwrap(), REASON_EXP);
//         }
//     }

//     #[test]
//     fn parses_ids_without_reason() {
//         let input_strs = vec![
//             // Comma separated
//             "145764790046818304,193163974471188480,151018674793349121",
//             // Mentions
//             "<@145764790046818304> <@193163974471188480> <@151018674793349121>",
//             // Space separated
//             "145764790046818304 193163974471188480 151018674793349121 ",
//             // Random spaces
//             "145764790046818304   193163974471188480     151018674793349121              ",
//         ];

//         for s in input_strs {
//             let args = Args::new(s, &[Delimiter::Single(' ')]);

//             let (ids, reason) = parse_id_reason(args);

//             assert_eq!(ids, IDS_EXP);
//             assert!(reason.is_none());
//         }
//     }

//     #[test]
//     fn parse_ids_dedups() {
//         let input_strs = vec![
//             // Comma separated
//             "145764790046818304,193163974471188480,151018674793349121,151018674793349121,151018674793349121,151018674793349121",
//             // Mentions
//             "<@145764790046818304> <@193163974471188480> <@151018674793349121><@151018674793349121><@151018674793349121>",
//         ];

//         for s in input_strs {
//             let args = Args::new(s, &[Delimiter::Single(' ')]);

//             let (ids, reason) = parse_id_reason(args);

//             assert_eq!(ids.len(), 3);
//             assert_eq!(ids, IDS_EXP);
//             assert!(reason.is_none());
//         }
//     }

//     #[test]
//     fn parse_ids_ignores_ids_in_reason() {
//         let inputs_and_expected = vec![
//             // Comma separated
//             (
//                 "145764790046818304,193163974471188480,151018674793349121 some reason 193163974471188480 text",
//                 "some reason 193163974471188480 text"
//             ),
//             // Mentions
//             (
//                 "<@145764790046818304> <@193163974471188480> <@151018674793349121> some reason <@193163974471188480> text",
//                 "some reason <@193163974471188480> text"
//             ),
//             // Space separated
//             (
//                 "145764790046818304 193163974471188480 151018674793349121 some 193163974471188480 reason text",
//                 "some 193163974471188480 reason text"
//             ),
//             // Random spacing
//             (
//                 "145764790046818304   193163974471188480    151018674793349121 some reason 193163974471188480    text",
//                 "some reason 193163974471188480    text"
//             ),
//         ];

//         for (input, expected_reason) in inputs_and_expected {
//             let args = Args::new(input, &[Delimiter::Single(' ')]);

//             let (ids, reason) = parse_id_reason(args);

//             assert_eq!(ids, IDS_EXP);
//             assert_eq!(reason.unwrap(), expected_reason);
//         }
//     }

//     #[test]
//     fn parses_ids_reason_duration() {
//         let input_strs = vec![
//             "145764790046818304,193163974471188480,151018674793349121 some reason text 6 hours 4 minutes 2 seconds",
//             "145764790046818304,193163974471188480,151018674793349121 some reason text 6hours 4minutes 2seconds",
//             "145764790046818304,193163974471188480,151018674793349121 some reason text 6h 4m 2s",
//             "145764790046818304,193163974471188480,151018674793349121 6hrs 4m 2secs some reason text ",
//             // right in the middle wat the
//             "145764790046818304,193163974471188480,151018674793349121 some 6h 4m 2s reason text",
//         ];

//         // 6 hours 4 minutes 2 seconds
//         let duration_exp = Duration::seconds((3600 * 6) + (60 * 4) + 2);

//         for s in input_strs {
//             let args = Args::new(s, &[Delimiter::Single(' ')]);

//             let (ids, reason, duration) = parse_id_reason_duration(args);

//             assert_eq!(ids, IDS_EXP);
//             assert_eq!(reason.unwrap(), REASON_EXP);
//             assert_eq!(duration.unwrap().unwrap(), duration_exp);
//         }
//     }

//     #[test]
//     fn parses_ids_duration_no_reason() {
//         // Whitespace should NOT count as the reason, empty reason would cause
//         // Discord embeds to fail
//         let input_strs = vec![
//             "145764790046818304,193163974471188480,151018674793349121 6 hours 4 minutes 2 seconds",
//             "145764790046818304,193163974471188480,151018674793349121 6hours 4minutes 2seconds",
//             "145764790046818304,193163974471188480,151018674793349121 6h 4m 2s",
//             "145764790046818304,193163974471188480,151018674793349121 6hrs 4m 2secs ",
//             "145764790046818304,193163974471188480,151018674793349121  6h 4m 2s",
//         ];

//         // 6 hours 4 minutes 2 seconds
//         let duration_exp = Duration::seconds((3600 * 6) + (60 * 4) + 2);

//         for s in input_strs {
//             let args = Args::new(s, &[Delimiter::Single(' ')]);

//             let (ids, reason, duration) = parse_id_reason_duration(args);

//             assert_eq!(ids, IDS_EXP);
//             println!("reason: {:#?}", reason);
//             assert!(reason.is_none());
//             assert_eq!(duration.unwrap().unwrap(), duration_exp);
//         }
//     }

//     #[test]
//     fn parses_ids_reason_no_duration() {
//         let input_strs = vec![
//             "145764790046818304,193163974471188480,151018674793349121 some reason text",
//             "145764790046818304,193163974471188480,151018674793349121   some reason text    ",
//         ];

//         for s in input_strs {
//             let args = Args::new(s, &[Delimiter::Single(' ')]);

//             let (ids, reason, duration) = parse_id_reason_duration(args);

//             assert_eq!(ids, IDS_EXP);
//             assert!(duration.is_none());
//             assert_eq!(reason.unwrap(), REASON_EXP);
//         }
//     }

//     #[test]
//     fn parses_ids_reason_excludes_non_user() {
//         let input_strs = vec![
//             "145764790046818304,193163974471188480,151018674793349121 <#151018674793349121> test",
//             "<@145764790046818304> <@193163974471188480> <@151018674793349121> <@&151018674793349121> a",
//             "145764790046818304 193163974471188480 151018674793349121 <#151018674793349121> reason",
//             "145764790046818304   193163974471188480    151018674793349121 <@&151018674793349121> bunbunbunbun",
//         ];

//         let reason_exp = vec![
//             "<#151018674793349121> test",
//             "<@&151018674793349121> a",
//             "<#151018674793349121> reason",
//             "<@&151018674793349121> bunbunbunbun",
//         ];

//         for (s, reason_exp) in input_strs.iter().zip(reason_exp.iter()) {
//             let args = Args::new(s, &[Delimiter::Single(' ')]);

//             let (ids, reason, duration) = parse_id_reason_duration(args);

//             assert_eq!(ids, IDS_EXP);
//             assert!(duration.is_none());
//             assert_eq!(reason.unwrap(), *reason_exp);
//         }
//     }
// }
