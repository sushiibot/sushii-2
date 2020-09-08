use serenity::framework::standard::{macros::command, Args, CommandResult};
use serenity::model::prelude::*;
use serenity::prelude::*;

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

    let roles_conf_str = {
        let mut r = args.rest();
        // Remove codeblock backticks
        r = r.trim_start_matches("```json");
        r = r.trim_start_matches("```toml");
        r = r.trim_start_matches('`');
        r = r.trim_end_matches('`');

        r.trim()
    };

    if roles_conf_str.is_empty() {
        msg.channel_id
            .say(&ctx.http, "Please provide a roles config")
            .await?;

        return Ok(());
    }

    let config_type_hint = if roles_conf_str.starts_with('{') {
        Some(ConfigType::Json)
    } else if roles_conf_str.starts_with('[') {
        Some(ConfigType::Toml)
    } else {
        None
    };

    let roles_conf = if let Some(hint) = config_type_hint {
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
                }),
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
                }),
        };

        match roles_conf {
            Err(e) => {
                msg.channel_id
                    .say(&ctx.http, e)
                    .await?;

                return Ok(());
            },
            Ok(c) => {
                // serde_json::value::to_value shouldn't fail if it successfully
                // the serialized before, but returning early instead of
                // unwrapping just in case 
                c?
            },
        }
    } else {
        msg.channel_id
            .say(&ctx.http, "Please double check your configuratio")
            .await?;
        
        return Ok(());
    };

    conf.role_config.replace(roles_conf);

    conf.save(&ctx).await?;

    msg.channel_id
        .say(&ctx.http, "Updated the roles configuration")
        .await?;

    Ok(())
}
