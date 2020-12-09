use futures::join;
use lazy_static::lazy_static;
use regex::{Regex, RegexBuilder};
use serenity::{model::prelude::*, prelude::*};
use std::collections::{HashMap, HashSet};
use std::fmt::Write;
use std::time::Duration;
use std::vec::Vec;
use tokio::time::delay_for;

use crate::error::Result;
use crate::keys::CacheAndHttpContainer;
use crate::model::sql::*;

#[derive(Clone, Debug, Eq, PartialEq)]
enum RoleActionKind {
    Add,
    Remove,
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct RoleAction {
    pub index: usize,
    pub kind: RoleActionKind,
    pub role_name: String,
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
struct CalculatedRoles<'a> {
    pub member_new_all_roles: HashSet<u64>,
    pub added_role_names: Vec<&'a str>,
    pub removed_role_names: Vec<&'a str>,
    pub added_existing_roles: Vec<&'a str>,
    pub removed_missing_roles: Vec<&'a str>,
    pub over_limit_roles: HashMap<&'a str, Vec<&'a str>>,
}

fn pluralize(s: &str, qty: usize) -> String {
    if qty > 1 {
        return format!("{}s", s);
    }

    s.into()
}

fn vec_to_code_string<S: AsRef<str>, I: IntoIterator<Item = S>>(v: I) -> String {
    v.into_iter()
        .map(|s| format!("`{}`", s.as_ref()))
        .collect::<Vec<String>>()
        .join(", ")
}

pub async fn message(ctx: &Context, msg: &Message) {
    let guild_id = match msg.guild_id {
        Some(id) => id,
        None => return,
    };

    // Return if an error happens or if not in role channel
    let role_channel = match GuildConfig::from_id(&ctx, &guild_id)
        .await
        .ok()
        .flatten()
        .and_then(|c| c.role_channel)
    {
        Some(c) => c,
        None => return,
    };

    if msg.channel_id.0 != role_channel as u64 {
        return;
    }

    match _message(ctx, msg).await {
        Ok(msg_string) => {
            let msg_string = match msg_string {
                Some(s) => s,
                None => return, // Not in role channel or failed some other pre-check
            };

            let sent_msg = msg.channel_id.say(&ctx.http, &msg_string).await;

            if let Err(e) = sent_msg.as_ref() {
                tracing::warn!(message=%msg_string, "Failed to send role message: {}", e);
            };

            // Delete messages after 10 seconds
            delay_for(Duration::from_secs(10)).await;

            let data = &ctx.data.read().await;
            let cache_http = data.get::<CacheAndHttpContainer>().unwrap();

            if let Ok(sent_msg) = sent_msg {
                // Run both delete futures concurrently instead of in series
                // try_join! better for Results but still want to try deleting both as
                // try_join! short circuits and returns immediately on any Error
                let (recv_res, sent_res) =
                    join!(msg.delete(&cache_http), sent_msg.delete(&cache_http));

                if let Err(e) = recv_res {
                    tracing::warn!(?msg, "Failed to delete received message: {}", e);
                }

                if let Err(e) = sent_res {
                    tracing::warn!(message=%msg_string, "Failed to delete sent message: {}", e);
                }
            } else {
                // Role message failed sooo just delete user's message
                let _ = msg.delete(&cache_http).await;
            }
        }
        Err(e) => {
            // This should only run if message sent in the role channel,
            // otherwise sushii will delete any messages that have an error
            tracing::error!(?msg, "Failed to handle roles message: {}", e);

            let sent_msg = msg
                .channel_id
                .say(&ctx.http, "Failed to update your roles :(")
                .await;

            delay_for(Duration::from_secs(5)).await;

            let data = &ctx.data.read().await;
            let cache_http = data.get::<CacheAndHttpContainer>().unwrap();

            if let Ok(sent_msg) = sent_msg {
                // Ignore errors whatever
                let _ = join!(msg.delete(&ctx), sent_msg.delete(&ctx));
            } else {
                let _ = msg.delete(&cache_http).await;
            }
        }
    }
}

// searching for multiple role assignments
lazy_static! {
    static ref RE: Regex = RegexBuilder::new(r"(-|\+)([\w ]+)")
        .case_insensitive(true)
        .build()
        .unwrap();
}

fn parse_role_actions(s: &str) -> Vec<RoleAction> {
    // Vec<("+/-", "role name")
    RE.captures_iter(&s)
        .enumerate()
        .map(|(index, caps)| {
            // Should be safe to unwrap since both groups are required
            let kind = if caps.get(1).map(|m| m.as_str()).unwrap() == "+" {
                RoleActionKind::Add
            } else {
                RoleActionKind::Remove
            };
            let role_name = caps
                .get(2)
                .map(|m| m.as_str().trim().to_lowercase())
                .unwrap();

            RoleAction {
                index,
                kind,
                role_name,
            }
        })
        .collect()
}

fn categorize_member_roles<'a, I, R>(
    role_config: &'a GuildRoles,
    member_roles: I,
    is_reset: bool,
) -> (HashSet<u64>, HashMap<&'a str, HashSet<u64>>)
where
    I: IntoIterator<Item = R>,
    R: Into<u64>,
{
    // All roles of the member, this is modified on role add / remove
    // Use iterator instead of member so it's easier to test
    let mut member_all_roles: HashSet<u64> = member_roles.into_iter().map(|x| x.into()).collect();

    // Member roles that are in the role config <Category, Vec<role>>
    let mut member_config_roles: HashMap<&str, HashSet<u64>> = HashMap::new();

    // add the member's current roles
    for group in &role_config.groups {
        let group_entry = member_config_roles
            .entry(&group.name)
            .or_insert_with(HashSet::new);

        for role in group.roles.iter() {
            if member_all_roles.contains(&role.primary_id) {
                group_entry.insert(role.primary_id);
            }

            if let Some(secondary_id) = role.secondary_id {
                if member_all_roles.contains(&secondary_id) {
                    group_entry.insert(secondary_id);
                }
            }

            // remove role if removing all
            if is_reset {
                if member_all_roles.contains(&role.primary_id) {
                    member_all_roles.remove(&role.primary_id);
                }

                if let Some(id) = role.secondary_id {
                    if member_all_roles.contains(&id) {
                        member_all_roles.remove(&id);
                    }
                }
            }
        }
    }

    (member_all_roles, member_config_roles)
}

/// Returns HashMap<String role name, (role_name, role, group_name, group index)
fn build_role_name_map<'a>(
    role_config: &'a GuildRoles,
) -> HashMap<String, (&str, &'a GuildRole, &'a str, usize)> {
    // Config roles: map from role name -> (role, group_name)
    let mut role_name_map: HashMap<String, (&str, &GuildRole, &str, usize)> = HashMap::new();
    for (i, group) in role_config.groups.iter().enumerate() {
        for role in &group.roles {
            role_name_map.insert(
                role.name.trim().to_lowercase(),
                (&role.name, &role, &group.name, i),
            );
        }
    }

    role_name_map
}

fn dedupe_role_actions<'a>(role_actions: &'a [RoleAction]) -> Vec<&'a RoleAction> {
    tracing::debug!(?role_actions, "role_actions");

    // Not the actual "dedupe" but more to check if a user is adding/removing a
    // same role.
    let mut role_actions_deduped_map: HashMap<&str, &RoleAction> = HashMap::new();

    for action in role_actions {
        let opposite_kind = if action.kind == RoleActionKind::Add {
            RoleActionKind::Remove
        } else {
            RoleActionKind::Add
        };

        // Remove the opposite action for same roles
        // eg: If there's already a +role1, adding a -role1 will remove the +role1 and vice versa
        // If there's a +role1, adding +role1 will just keep the first one
        if let Some(val) = role_actions_deduped_map.get(action.role_name.as_str()) {
            tracing::debug!(?action, ?val, "Found duplicate role");

            // Remove existing if there's an opposite
            if val.kind == opposite_kind {
                role_actions_deduped_map.remove(action.role_name.as_str());
            }

            // Don't do anything if there's a same
            continue;
        }

        // No existing same role, now can add it
        role_actions_deduped_map.insert(&action.role_name, action);
    }

    let mut role_actions_deduped: Vec<&RoleAction> =
        role_actions_deduped_map.into_iter().map(|r| r.1).collect();

    role_actions_deduped.sort_by(|a, b| a.index.cmp(&b.index));

    tracing::debug!(?role_actions_deduped, "role_actions_deduped");

    role_actions_deduped
}

fn calculate_roles<'a>(
    role_config: &GuildRoles,
    role_actions_deduped: Vec<&'a RoleAction>,
    role_name_map: HashMap<String, (&'a str, &'a GuildRole, &'a str, usize)>,
    mut member_all_roles: HashSet<u64>,
    mut member_config_roles: HashMap<&'a str, HashSet<u64>>,
) -> CalculatedRoles<'a> {
    let mut added_role_names: Vec<&str> = Vec::new();
    let mut removed_role_names: Vec<&str> = Vec::new();

    let mut added_existing_roles: Vec<&str> = Vec::new();
    let mut removed_missing_roles: Vec<&str> = Vec::new();
    let mut over_limit_roles: HashMap<&str, Vec<&str>> = HashMap::new();

    for action in role_actions_deduped {
        if let Some((orig_role_name, role, group_name, group_index)) =
            role_name_map.get(action.role_name.trim())
        {
            // Member's current roles in this group
            let cur_group_roles = member_config_roles
                .entry(group_name)
                .or_insert_with(HashSet::new);

            let conf_group = role_config.groups.get(*group_index).unwrap();

            if action.kind == RoleActionKind::Add {
                // If member already has it
                if cur_group_roles.contains(&role.primary_id) {
                    added_existing_roles.push(orig_role_name);

                    continue;
                }

                // Check limits if limit is set to greater than 0 (0 is disabled)
                if conf_group.limit > 0 && cur_group_roles.len() >= conf_group.limit as usize {
                    let entry = over_limit_roles.entry(group_name).or_insert_with(Vec::new);

                    entry.push(orig_role_name);

                    continue;
                }

                // Add role
                let id_to_add = if cur_group_roles.is_empty() || role.secondary_id.is_none() {
                    role.primary_id
                } else if let Some(id) = role.secondary_id {
                    id
                } else {
                    // Shouldn't reach here but ok just skip
                    continue;
                };

                // This is just to keep track of roles and limits
                cur_group_roles.insert(id_to_add);
                // This is a set to actually send to Discord API of **all**
                // the user's role to update to, as this may include roles
                // not in the role config
                member_all_roles.insert(id_to_add);
                added_role_names.push(orig_role_name);
            } else {
                let has_role = cur_group_roles.contains(&role.primary_id)
                    || role
                        .secondary_id
                        .map_or(false, |id| cur_group_roles.contains(&id));

                if !has_role {
                    removed_missing_roles.push(orig_role_name);

                    continue;
                }

                cur_group_roles.remove(&role.primary_id);
                member_all_roles.remove(&role.primary_id);

                if let Some(secondary_id) = role.secondary_id {
                    cur_group_roles.remove(&secondary_id);
                    member_all_roles.remove(&secondary_id);
                }

                removed_role_names.push(orig_role_name);
            }
        }
    }

    CalculatedRoles {
        // Just give the same HashSet ownership back I guess lol
        member_new_all_roles: member_all_roles,
        added_role_names,
        removed_role_names,
        added_existing_roles,
        removed_missing_roles,
        over_limit_roles,
    }
}

fn format_response(role_config: &GuildRoles, calc_roles: &CalculatedRoles) -> String {
    let CalculatedRoles {
        added_role_names,
        removed_role_names,
        added_existing_roles,
        removed_missing_roles,
        over_limit_roles,
        ..
    } = calc_roles;

    let mut s = String::new();

    if !added_role_names.is_empty() {
        let _ = writeln!(
            s,
            "Added {}: {}",
            pluralize("role", added_role_names.len()),
            vec_to_code_string(added_role_names)
        );
    }

    if !removed_role_names.is_empty() {
        let _ = writeln!(
            s,
            "Removed {}: {}",
            pluralize("role", removed_role_names.len()),
            vec_to_code_string(removed_role_names)
        );
    }

    // Respond attempt to add role user already has
    if !added_existing_roles.is_empty() {
        let _ = writeln!(
            s,
            "You already have the following {} so they were not added: {}",
            pluralize("role", added_existing_roles.len()),
            vec_to_code_string(added_existing_roles)
        );
    }

    // Respond attempt to remove role user doens't have
    if !removed_missing_roles.is_empty() {
        let _ = writeln!(
            s,
            "Cannot remove {} you do not have: {}",
            pluralize("role", removed_missing_roles.len()),
            vec_to_code_string(removed_missing_roles)
        );
    }

    // Check if there are over limit roles
    if !over_limit_roles.is_empty() {
        let _ = write!(s, "Cannot add roles that exceed role group limits: ",);
    }

    for (group_name, role_names) in over_limit_roles {
        if let Some(group) = &role_config.groups.iter().find(|&x| x.name == *group_name) {
            let _ = writeln!(
                s,
                "{} (`{}` group has a limit of `{}` {})",
                vec_to_code_string(role_names),
                group_name,
                group.limit,
                pluralize("role", group.limit as usize),
            );
        }
    }

    s
}

pub async fn _message(ctx: &Context, msg: &Message) -> Result<Option<String>> {
    // ignore self
    if msg.is_own(&ctx).await {
        return Ok(None);
    }

    // Delete other bots and return
    if msg.author.bot {
        msg.delete(&ctx).await?;

        return Ok(None);
    }

    let guild = match msg.guild(&ctx.cache).await {
        Some(g) => g,
        None => {
            return Ok(None);
        }
    };

    let guild_conf = match GuildConfig::from_id(&ctx, &guild.id).await? {
        Some(c) => c,
        None => {
            tracing::error!(?msg, "No guild config found while handling roles");
            return Ok(None);
        }
    };

    // get configs
    let role_config: GuildRoles = match guild_conf.role_config {
        Some(c) => match serde_json::from_value(c) {
            Ok(c) => c,
            Err(e) => {
                tracing::warn!(
                    guild_id = guild.id.0,
                    "Failed to convert guild role config to GuildRoles struct: {}",
                    e
                );

                // If role deserialize fails, respond with error
                return Ok(Some("Role configuration is invalid".into()));
            }
        },
        None => return Ok(None),
    };

    // Kind of redundant since already checked if in role channel before but oh well
    let role_channel = match guild_conf.role_channel {
        Some(c) => c,
        None => return Ok(None),
    };

    // check if in correct channel
    if msg.channel_id.0 != role_channel as u64 {
        return Ok(None);
    }

    let data = &ctx.data.read().await;
    let cache_http = data.get::<CacheAndHttpContainer>().unwrap();

    if !RE.is_match(&msg.content) && msg.content != "clear" && msg.content != "reset" {
        return Ok(Some("You can add a role with `+role name` or remove a role with `-role name`.  Use `clear` or `reset` to remove all roles".into()));
    }

    // Vec<("+/-", "role name")
    let role_actions = parse_role_actions(&msg.content);
    // Not the actual "dedupe" but more to check if a user is adding/removing a
    // same role.
    let role_actions_deduped = dedupe_role_actions(&role_actions);

    // Should remove all roles
    let is_reset = msg.content == "clear" || msg.content == "reset";

    let member = guild
        .member(&cache_http, msg.author.id)
        .await
        .map_err(|e| {
            tracing::error!("Failed to fetch guild member: {}", e);
            e
        })?;

    let (member_all_roles, member_config_roles) =
        categorize_member_roles(&role_config, member.roles.clone(), is_reset);
    let role_name_map = build_role_name_map(&role_config);

    let calc_roles = calculate_roles(
        &role_config,
        role_actions_deduped,
        role_name_map,
        member_all_roles,
        member_config_roles,
    );

    let s = format_response(&role_config, &calc_roles);

    // After all checks if the responding string is empty then all previous ones are empty
    if s.is_empty() && !is_reset {
        return Ok(Some("Couldn't modify your roles. You can add a role with `+role name` or remove a role with `-role name`.  Use `clear` or `reset` to remove all roles".into()));
    }

    let before_roles: HashSet<_> = member.roles.iter().map(|r| r.0).collect();
    let after_roles: HashSet<_> = calc_roles
        .member_new_all_roles
        .iter()
        .map(|id| *id)
        .collect();

    // Only edit member if there are roles added or removed
    if before_roles != after_roles {
        // If edit member fails, just return error
        let _ = guild
            .edit_member(&ctx.http, msg.author.id, |m| {
                m.roles(
                    &calc_roles
                        .member_new_all_roles
                        .iter()
                        .map(|i| RoleId(*i))
                        .collect::<Vec<RoleId>>(),
                )
            })
            .await
            .map_err(|e| {
                tracing::error!(?calc_roles, "Failed to edit member: {}", e);

                e
            });
    }

    if is_reset {
        return Ok(Some("Your roles have been reset.".into()));
    }

    Ok(Some(s))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn role_conf() -> GuildRoles {
        GuildRoles {
            groups: vec![
                GuildGroup {
                    name: "FirstGroup".into(),
                    limit: 2,
                    roles: vec![
                        GuildRole {
                            name: "FirstRole".into(),
                            primary_id: 1,
                            secondary_id: None,
                        },
                        GuildRole {
                            name: "SecondRole".into(),
                            primary_id: 2,
                            secondary_id: Some(20),
                        },
                        GuildRole {
                            name: "ThirdRole".into(),
                            primary_id: 3,
                            secondary_id: Some(30),
                        },
                    ],
                },
                GuildGroup {
                    name: "SecondGroup".into(),
                    limit: 0,
                    roles: vec![GuildRole {
                        name: "Dog".into(),
                        primary_id: 100,
                        secondary_id: Some(1000),
                    }],
                },
            ],
        }
    }

    #[test]
    fn parses_role_actions() {
        let strs = vec![
            "+doggo -catto +bunno -catto +catto -bun",
            "+doggo -catto +bunno +catto -bun",
        ];

        for s in strs {
            let role_actions = parse_role_actions(&s);
            let role_actions_deduped = dedupe_role_actions(&role_actions);
            assert_eq!(role_actions_deduped.len(), 3);
        }
    }

    fn test_calc_roles(roles: Vec<u64>, s: &str, calc_roles_exp: CalculatedRoles) {
        let role_config = role_conf();

        let role_actions = parse_role_actions(&s);
        let role_actions_deduped = dedupe_role_actions(&role_actions);

        let (member_all_roles, member_config_roles) =
            categorize_member_roles(&role_config, roles, false);
        let role_name_map = build_role_name_map(&role_config);

        let calc_roles = calculate_roles(
            &role_config,
            role_actions_deduped,
            role_name_map,
            member_all_roles,
            member_config_roles,
        );

        assert_eq!(calc_roles, calc_roles_exp);
    }

    #[test]
    fn calculates_roles_ignores_nonexistant() {
        test_calc_roles(
            // 2 ids that don't exist in role config
            vec![0u64, 369u64],
            "+FirstRole -SecondRole +ThirdRole +NonExistantRole",
            CalculatedRoles {
                member_new_all_roles: [0, 369, 1, 30].iter().cloned().collect(),
                added_role_names: vec!["FirstRole", "ThirdRole"],
                removed_missing_roles: vec!["SecondRole"],
                ..Default::default()
            },
        );
    }

    #[test]
    fn calculates_roles_handles_repeated() {
        test_calc_roles(
            // repeated inputs
            vec![0u64],
            "+FirstRole +FirstRole +FirstRole",
            CalculatedRoles {
                member_new_all_roles: [0, 1].iter().cloned().collect(),
                added_role_names: vec!["FirstRole"],
                ..Default::default()
            },
        );
    }

    #[test]
    fn calculates_roles_handles_secondary_ids() {
        test_calc_roles(
            // secondary ids
            vec![],
            "+FirstRole +SecondRole",
            CalculatedRoles {
                member_new_all_roles: [1, 20].iter().cloned().collect(),
                added_role_names: vec!["FirstRole", "SecondRole"],
                ..Default::default()
            },
        );
    }

    #[test]
    fn calculates_roles_handles_limits_all() {
        test_calc_roles(
            // role limits all at once
            vec![],
            "+FirstRole +SecondRole +ThirdRole +NonExistantRole",
            CalculatedRoles {
                member_new_all_roles: [1, 20].iter().cloned().collect(),
                added_role_names: vec!["FirstRole", "SecondRole"],
                over_limit_roles: [("FirstGroup", vec!["ThirdRole"])]
                    .iter()
                    .cloned()
                    .collect(),
                ..Default::default()
            },
        );
    }

    #[test]
    fn calculates_roles_handles_limits_new() {
        test_calc_roles(
            // role limits all at once
            vec![1, 20],
            "+ThirdRole +NonExistantRole",
            CalculatedRoles {
                member_new_all_roles: [1, 20].iter().cloned().collect(),
                over_limit_roles: [("FirstGroup", vec!["ThirdRole"])]
                    .iter()
                    .cloned()
                    .collect(),
                ..Default::default()
            },
        );
    }
}
