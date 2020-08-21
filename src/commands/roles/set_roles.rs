use serenity::framework::standard::{macros::command, Args, CommandResult};
use serenity::model::prelude::*;
use serenity::prelude::*;

use crate::model::sql::*;

#[command]
async fn set(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    let mut conf = GuildConfig::from_msg_or_respond(&ctx, msg).await?;

    let roles_conf_str = {
        let mut r = args.rest();
        // Remove codeblock backticks
        r = r.trim_start_matches("```json");
        r = r.trim_start_matches('`');
        r = r.trim_end_matches('`');

        r
    };

    if roles_conf_str.is_empty() {
        msg.channel_id
            .say(&ctx.http, "Please provide a roles config")
            .await?;

        return Ok(());
    }

    // Try deserializing to verify configuration
    let roles_conf: GuildRoles = match serde_json::from_str(&roles_conf_str) {
        Ok(conf) => conf,
        Err(e) => {
            // Column can be 0, so saturating substract 1 to get correct position
            let arrow = format!("{}^", " ".repeat(e.column().saturating_sub(1)));

            // Add an arrow ^ to where the error is
            let mut conf_str_arr = roles_conf_str.split('\n').collect::<Vec<&str>>();
            conf_str_arr.insert(e.line(), &arrow);

            msg.channel_id
                .say(
                    &ctx.http,
                    format!(
                        "Error in roles config: {}\n\
                        ```json\n{}\n```",
                        e,
                        conf_str_arr.join("\n")
                    ),
                )
                .await?;

            return Ok(());
        }
    };

    let roles_conf_value = serde_json::to_value(roles_conf)?;

    conf.role_config.replace(roles_conf_value);

    conf.save(&ctx).await?;

    msg.channel_id
        .say(&ctx.http, "Updated the roles configuration")
        .await?;

    Ok(())
}
