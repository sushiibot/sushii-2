macro_rules! settings_cmds {
    ($cmd_name:ident, $settings_field:ident, $settings_field_enabled:ident, $set_expr:expr) => {
        use serenity::framework::standard::{macros::command, Args, CommandResult};
        use serenity::model::prelude::*;
        use serenity::prelude::*;

        use crate::model::sql::*;

        #[command]
        #[sub_commands(set, on, off, toggle)]
        async fn $cmd_name(ctx: &Context, msg: &Message) -> CommandResult {
            let _ = msg
                .channel_id
                .say(
                    &ctx.http,
                    format!(
                        "Available sub-commands for `{}` are `set`, `on`, `off`, and `toggle`",
                        stringify!($cmd_name)
                    ),
                )
                .await?;

            Ok(())
        }

        /// Set the moderation log channel
        #[command]
        async fn set(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
            let mut conf = GuildConfig::from_msg_or_respond(&ctx, msg).await?;

            let join_msg = $set_expr;

            conf.$settings_field.replace(join_msg.to_string());
            conf.save(&ctx).await?;

            msg.channel_id
                .say(&ctx.http, format!("Join message to: {}", join_msg))
                .await?;

            Ok(())
        }

        /// Turns off moderation log
        #[command]
        async fn off(ctx: &Context, msg: &Message) -> CommandResult {
            let mut conf = GuildConfig::from_msg_or_respond(&ctx, msg).await?;

            if !conf.$settings_field_enabled {
                let _ = msg
                    .channel_id
                    .say(&ctx.http, "Error: Join messages are already off")
                    .await?;

                return Ok(());
            }

            conf.$settings_field_enabled = false;
            conf.save(&ctx).await?;

            let _ = msg
                .channel_id
                .say(
                    &ctx.http,
                    "<:offline:316354467031416832> Turned off join messages",
                )
                .await?;

            Ok(())
        }

        /// Turns on join messages
        #[command]
        async fn on(ctx: &Context, msg: &Message) -> CommandResult {
            let mut conf = GuildConfig::from_msg_or_respond(&ctx, msg).await?;

            if conf.$settings_field_enabled {
                let _ = msg
                    .channel_id
                    .say(&ctx.http, "Error: Join messages are already on")
                    .await?;

                return Ok(());
            }

            conf.$settings_field_enabled = true;
            conf.save(&ctx).await?;

            let _ = msg
                .channel_id
                .say(
                    &ctx.http,
                    "<:online:316354435745972244> Turned on join messages",
                )
                .await?;

            Ok(())
        }

        /// Toggles join messages
        #[command]
        async fn toggle(ctx: &Context, msg: &Message) -> CommandResult {
            let mut conf = GuildConfig::from_msg_or_respond(&ctx, msg).await?;

            conf.$settings_field_enabled = !conf.$settings_field_enabled;
            conf.save(&ctx).await?;

            let on_or_off = if conf.$settings_field_enabled {
                ("<:online:316354435745972244>", "on")
            } else {
                ("<:offline:316354467031416832>", "off")
            };

            let _ = msg
                .channel_id
                .say(
                    &ctx.http,
                    format!("{} Toggled join messages `{}`", on_or_off.0, on_or_off.1),
                )
                .await?;

            Ok(())
        }
    };
}
