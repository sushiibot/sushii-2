use serenity::framework::standard::{macros::command, Args, CommandResult};
use serenity::model::prelude::*;
use serenity::prelude::*;

use crate::model::sql::*;

#[command]
#[required_permissions("MANAGE_GUILD")]
async fn get(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let conf = GuildConfig::from_msg_or_respond(&ctx, msg).await?;

    let format = args
        .single::<String>()
        .unwrap_or_else(|_| "json".to_string());

    if conf.role_config.is_none() {
        msg.channel_id
            .say(&ctx.http, "There isn't a role configuration set.")
            .await?;

        return Ok(());
    }

    let conf_str = match format.as_ref() {
        "json" => serde_json::to_string_pretty(&conf.role_config)?,
        "yml" | "yaml" => serde_yaml::to_string(&conf.role_config)?,
        _ => {
            msg.channel_id
                .say(
                    &ctx.http,
                    "Invalid format, valid options are `json` or `yaml`",
                )
                .await?;

            return Ok(());
        }
    };

    msg.channel_id
        .say(
            &ctx.http,
            format!(
                "Current roles configuration:\n```{}\n{}\n```",
                format, conf_str
            ),
        )
        .await?;

    Ok(())
}
