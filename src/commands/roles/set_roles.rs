use serenity::framework::standard::{macros::command, Args, CommandResult};
use serenity::model::prelude::*;
use serenity::prelude::*;
use serde_json::value::Value;

use crate::model::sql::*;

enum ConfigType {
    Json,
    Toml,
}

fn error_pointed_str(s: &str, line: usize, col: usize) -> String {
    // Column can be 0, so saturating substract 1 to get correct position
    let arrow = format!("{}^", " ".repeat(col.saturating_sub(1)));

    // Add an arrow ^ to where the error is
    let mut str_arr = s.split('\n').collect::<Vec<&str>>();
    str_arr.insert(line, &arrow);

    str_arr.join("\n")
}

#[command]
async fn set(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    let mut conf = GuildConfig::from_msg_or_respond(&ctx, msg).await?;

    let roles_conf = match parse_config(&args.rest()) {
        Err(e) => {
            let _ = msg.channel_id.say(&ctx.http, &e).await?;

            return Ok(());
        },
        Ok(c) => c,
    };

    conf.role_config.replace(roles_conf);

    conf.save(&ctx).await?;

    msg.channel_id
        .say(&ctx.http, "Updated the roles configuration")
        .await?;

    Ok(())
}

fn parse_config(input_str: &str) -> Result<Value, String> {
    let roles_conf_str = {
        let mut input_str = input_str;

        // Remove codeblock backticks
        input_str = input_str.trim_start_matches("```json");
        input_str = input_str.trim_start_matches("```toml");
        input_str = input_str.trim_start_matches('`');
        input_str = input_str.trim_end_matches('`');

        input_str.trim()
    };

    if roles_conf_str.is_empty() {
        return Err("Please provide a roles config".into());
    }

    let config_type_hint = if roles_conf_str.starts_with('{') {
        Some(ConfigType::Json)
    } else if roles_conf_str.starts_with('[') {
        Some(ConfigType::Toml)
    } else {
        None
    };

    if let Some(hint) = config_type_hint {
        let roles_conf = match hint {
            ConfigType::Json => serde_json::from_str::<GuildRoles>(&roles_conf_str)
                .map(serde_json::to_value)
                .map_err(|e| {
                    format!(
                        "Error in roles config: {}\n\
                        ```json\n{}\n```",
                        e,
                        error_pointed_str(roles_conf_str, e.line(), e.column())
                    )
                })?,
            ConfigType::Toml => toml::from_str::<GuildRoles>(&roles_conf_str)
                .map(serde_json::to_value)
                .map_err(|e| {
                    if let Some((line, column)) = e.line_col() {
                        format!(
                            "Error in roles config: {}\n\
                            ```toml\n{}\n```",
                            e,
                            error_pointed_str(roles_conf_str, line, column)
                        )
                    } else {
                        "Invalid toml configuration".to_string()
                    }
                })?,
        };

        roles_conf.map_err(|_| "Failed to serialize configuration".into())
    } else {
        Err("Invalid roles configuration".into())
    }
}

#[test]
fn parses_roles_config_from_json() {
    let json_config_base = r#"{
        "bias": {
            "limit": 3,
            "roles": {
                "First Role": {
                    "primary_id": 123
                },
                "Second Role": {
                    "primary_id": 456,
                    "secondary_id": 789
                }
            }
        },
        "extra": {
            "roles": {
                "Third Role": {
                    "primary_id": 1011
                }
            }
        }
    }"#;

    let configs = vec![
        format!("```{}```", &json_config_base),
        format!("`{}`", &json_config_base),
        format!("```json{}```", &json_config_base),
        format!("```json\n{}```", &json_config_base),
        format!("```json{}`", &json_config_base),
        json_config_base.to_string(),
    ];

    for config in configs {
        let c = parse_config(&config);

        assert!(c.is_ok());
    }
}

#[test]
fn parses_roles_config_from_toml() {
    let toml_config_base = r#"
    [bias]
    limit = 3

    [bias.roles]
    [bias.roles."First Role"]
    primary_id = 123

    [bias.roles."Second Role"]
    primary_id = 456
    secondary_id = 789

    [extra]
    [extra.roles]
    [extra.roles."Third Role"]
    primary_id = 1011
    "#;

    let configs = vec![
        format!("```{}```", &toml_config_base),
        format!("`{}`", &toml_config_base),
        format!("```toml{}```", &toml_config_base),
        format!("```toml\n{}```", &toml_config_base),
        format!("```toml{}`", &toml_config_base),
        toml_config_base.to_string(),
    ];

    for config in configs {
        let c = parse_config(&config);

        assert!(c.is_ok());
    }
}
