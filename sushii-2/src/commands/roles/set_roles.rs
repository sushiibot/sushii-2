use serenity::framework::standard::{macros::command, Args, CommandResult};
use serenity::model::prelude::*;
use serenity::prelude::*;

use crate::model::sql::*;

enum ConfigType {
    Json,
    Yaml,
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
#[required_permissions("MANAGE_GUILD")]
async fn set(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    let mut conf = GuildConfig::from_msg_or_respond(&ctx, msg).await?;

    let conf_str = if msg.attachments.is_empty() {
        args.rest().to_string()
    } else if let Some(attachment) = msg.attachments.first() {
        String::from_utf8(attachment.download().await?)?
    } else {
        "".into()
    };

    let roles_conf = match parse_config(&conf_str) {
        Err(e) => {
            let _ = msg.channel_id.say(&ctx.http, &e).await?;

            return Ok(());
        }
        Ok(c) => c,
    };

    let conf_value = match serde_json::to_value(roles_conf) {
        Ok(c) => c,
        Err(e) => {
            tracing::error!("Failed to serialize role configuration to value: {}", e);

            msg.channel_id
                .say(&ctx.http, "Failed to serialize configuration")
                .await?;

            return Ok(());
        }
    };

    conf.role_config.replace(conf_value);

    conf.save(&ctx).await?;

    msg.channel_id
        .say(&ctx.http, "Updated the roles configuration")
        .await?;

    Ok(())
}

fn parse_config(input_str: &str) -> Result<GuildRoles, String> {
    let roles_conf_str = {
        let mut input_str = input_str;

        // Remove codeblock backticks
        input_str = input_str.trim_start_matches("```json");
        input_str = input_str.trim_start_matches("```yaml");
        input_str = input_str.trim_start_matches("```yml");
        input_str = input_str.trim_start_matches('`');
        input_str = input_str.trim_end_matches('`');

        input_str.trim()
    };

    if roles_conf_str.is_empty() {
        return Err("Please provide a roles config".into());
    }

    let config_type_hint = if roles_conf_str.starts_with('{') {
        Some(ConfigType::Json)
    } else {
        Some(ConfigType::Yaml)
    };

    if let Some(hint) = config_type_hint {
        match hint {
            ConfigType::Json => serde_json::from_str::<GuildRoles>(&roles_conf_str).map_err(|e| {
                format!(
                    "Error in roles config: {}\n\
                        ```json\n{}\n```",
                    e,
                    error_pointed_str(roles_conf_str, e.line(), e.column())
                )
            }),
            ConfigType::Yaml => serde_yaml::from_str::<GuildRoles>(&roles_conf_str).map_err(|e| {
                if let Some(location) = e.location() {
                    format!(
                        "Error in roles config: {}\n\
                            ```yaml\n{}\n```",
                        e,
                        error_pointed_str(roles_conf_str, location.line(), location.column())
                    )
                } else {
                    "Invalid yaml configuration".to_string()
                }
            }),
        }
    } else {
        Err("Invalid roles configuration".into())
    }
}

#[test]
fn parses_roles_config_from_json() {
    let json_config_base = r#"{
        "groups": [
            {
                "name": "bias",
                "limit": 3,
                "roles": [
                    {
                        "name": "First Role",
                        "primary_id": 123
                    },
                    {
                        "name": "Second Role",
                        "primary_id": 456,
                        "secondary_id": 789
                    }
                ]
            },
            {
                "name": "extra",
                "roles": [
                    {
                        "name": "Third Role",
                        "primary_id": 1011
                    }
                ]
            }
        ]
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

        println!("{:#?}", c);

        assert!(c.is_ok());
    }
}

#[test]
fn parses_roles_config_from_yaml() {
    let yaml_config_base = r#"
    groups:
    - name: bias
      limit: 3
      roles:
      - name: First Role
        primary_id: 123
      - name: Second Role
        primary_id: 456
        secondary_id: 789
    - name: extra
      roles:
      - name: Third Role
        primary_id: 1011
    "#;

    let configs = vec![
        format!("```{}```", &yaml_config_base),
        format!("`{}`", &yaml_config_base),
        format!("```yaml{}```", &yaml_config_base),
        format!("```yaml\n{}```", &yaml_config_base),
        format!("```yml{}`", &yaml_config_base),
        yaml_config_base.to_string(),
    ];

    for config in configs {
        let c = parse_config(&config);

        assert!(c.is_ok());
    }
}
