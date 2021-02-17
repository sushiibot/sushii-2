use serenity::framework::standard::{macros::command, Args, CommandResult};
use serenity::model::prelude::*;
use serenity::prelude::*;
use std::str::FromStr;

use crate::model::sql::*;

#[command]
#[required_permissions("MANAGE_GUILD")]
async fn default(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    // Default is either help message or modifying settings, set, on, off, toggle

    // First need to check for which setting to modify
    let setting_name = match args.single::<String>() {
        Ok(s) => s,
        Err(_) => {
            msg
                .channel_id
                .say(&ctx.http, "Available settings commands can be found here: <https://sushii.xyz/commands#settings>")
                .await?;

            return Ok(());
        }
    };

    let mut conf = GuildConfig::from_msg_or_respond(&ctx, msg).await?;

    let setting = match GuildSetting::from_str(&setting_name) {
        Ok(s) => s,
        Err(_) => {
            msg
                .channel_id
                .say(&ctx.http, "Error: Invalid setting. \
                    Available settings are: \n\
                    `joinmsg`, `joinreact`, `leavemsg`, `msgchannel`, `msglog`, `modlog`, `memberlog`, `mutedm`, `warndm`")
                .await?;

            return Ok(());
        }
    };

    let setting_action_str = match args.single::<String>() {
        Ok(s) => s,
        Err(_) => "show".into(),
    };

    let setting_action = match GuildSettingAction::from_str(&setting_action_str) {
        Ok(a) => a,
        Err(_) => {
            msg.channel_id
                .say(
                    &ctx.http,
                    "Error: Invalid setting action. \
                    Available actions are: `set`, `on`, `off`, `toggle`. \
                    To show the current setting, omit the action.",
                )
                .await?;

            return Ok(());
        }
    };

    let s = match setting_action {
        GuildSettingAction::Set => {
            let new_val = args.rest();

            match conf.set_setting(&setting, &new_val) {
                Ok(()) => format!("Updated {} to: {}", setting.to_string(), new_val),
                Err(e) => format!("Error: Invalid setting value, {}", e),
            }
        }
        GuildSettingAction::On => match conf.enable_setting(&setting) {
            Ok(true) => format!(
                "<:online:316354435745972244> Turned on {}",
                setting.to_string()
            ),
            Ok(false) => format!("Error: {} is already on", setting.to_string()),
            Err(e) => format!("Error: {}", e),
        },
        GuildSettingAction::Off => match conf.disable_setting(&setting) {
            Ok(true) => format!(
                "<:offline:316354467031416832> Turned off {}",
                setting.to_string()
            ),
            Ok(false) => format!("Error: {} is already off", setting.to_string()),
            Err(e) => format!("Error: {}", e),
        },
        GuildSettingAction::Toggle => match conf.toggle_setting(&setting) {
            Ok(true) => format!(
                "<:online:316354435745972244> Toggled on {}",
                setting.to_string()
            ),
            Ok(false) => format!(
                "<:offline:316354467031416832> Toggled off {}",
                setting.to_string()
            ),
            Err(e) => format!("Error: {}", e),
        },
        GuildSettingAction::Show => match conf.get_setting(&setting) {
            (Some(val), Some(true)) => format!(
                "<:online:316354435745972244> {} is **on** and currently set to: {}",
                setting.to_string(),
                val
            ),
            (Some(val), Some(false)) => format!(
                "<:offline:316354467031416832> {} is **off** and currently set to: {}",
                setting.to_string(),
                val
            ),
            (Some(val), None) => format!("{} is currently set to: {}", setting.to_string(), val),
            (None, _) => format!("{} is not set", setting.to_string()),
        },
    };

    conf.save(&ctx).await?;

    msg.channel_id.say(&ctx, s).await?;

    Ok(())
}
