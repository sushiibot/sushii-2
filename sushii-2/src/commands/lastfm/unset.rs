use serenity::framework::standard::{macros::command, CommandResult};
use serenity::model::prelude::*;
use serenity::prelude::*;

use crate::model::sql::*;

#[command]
#[only_in("guild")]
async fn unset(ctx: &Context, msg: &Message) -> CommandResult {
    let mut user_data = UserData::from_id_or_new(ctx, msg.author.id).await?;

    user_data.lastfm_username = None;
    user_data.save(ctx).await?;

    msg.channel_id
        .say(ctx, "Your Last.fm has been unset.")
        .await?;

    Ok(())
}
