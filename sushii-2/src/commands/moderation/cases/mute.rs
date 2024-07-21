use serenity::framework::standard::{macros::command, CommandResult};
use serenity::model::prelude::*;
use serenity::prelude::*;




#[command]
#[only_in("guild")]
#[required_permissions("BAN_MEMBERS")]
#[sub_commands(setduration, addduration)]
async fn mute(ctx: &Context, msg: &Message) -> CommandResult {
    msg.channel_id
        .say(
            &ctx.http,
            "Please use </timeout:996259097202671645> now or right-click the user -> timeout :)",
        )
        .await?;

    return Ok(());
}

#[command]
#[only_in("guild")]
#[required_permissions("BAN_MEMBERS")]
async fn unmute(ctx: &Context, msg: &Message) -> CommandResult {
    msg.channel_id
        .say(
            &ctx.http,
            "Please use </untimeout:1070531459229700147> now or right-click the user -> remove timeout :)",
        )
        .await?;

    return Ok(());
}

#[command]
#[only_in("guild")]
#[required_permissions("BAN_MEMBERS")]
#[aliases("s", "set", "setd", "setdur", "settime")]
async fn setduration(ctx: &Context, msg: &Message) -> CommandResult {
    msg.channel_id
        .say(
            &ctx.http,
            "Please use </timeout:996259097202671645> to update the duration of a mute :)",
        )
        .await?;

    return Ok(());
}

#[command]
#[only_in("guild")]
#[required_permissions("BAN_MEMBERS")]
#[aliases("a", "add", "addd", "adddur", "addtime", "extend")]
async fn addduration(ctx: &Context, msg: &Message) -> CommandResult {
    msg.channel_id
        .say(
            &ctx.http,
            "Please use </timeout:996259097202671645> to update the duration of a mute :)",
        )
        .await?;

    return Ok(());
}

