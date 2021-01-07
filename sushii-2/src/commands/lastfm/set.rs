use serenity::framework::standard::{macros::command, Args, CommandResult};
use serenity::model::prelude::*;
use serenity::prelude::*;

use crate::keys::*;
use crate::model::sql::*;

#[command]
#[only_in("guild")]
async fn set(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let username = match args.single::<String>() {
        Ok(s) => s,
        Err(_) => {
            msg.reply(ctx, "Error: Please give a Last.fm username")
                .await?;

            return Ok(());
        }
    };

    let mut user_data = UserData::from_id_or_new(ctx, msg.author.id).await?;

    let reqwest_client = ctx
        .data
        .read()
        .await
        .get::<ReqwestContainer>()
        .cloned()
        .unwrap();

    let sushii_conf = SushiiConfig::get(ctx).await;

    let mut fm_client =
        lastfm_rs::Client::from_reqwest_client(reqwest_client.clone(), &sushii_conf.lastfm_key);

    let lastfm_user = fm_client.user_info(&username).await.send().await?.user;

    metrics::increment_counter!("lastfm_api_queries", "endpoint" => "user.getInfo");

    user_data
        .lastfm_username
        .replace(lastfm_user.username.clone());
    user_data.save(ctx).await?;

    msg.channel_id
        .send_message(ctx, |m| {
            m.embed(|e| {
                e.title("Saved Last.fm user");
                e.author(|a| {
                    a.icon_url("https://i.imgur.com/C7u8gqg.jpg");
                    a.name(&lastfm_user.username);
                    a.url(&lastfm_user.url);

                    a
                });

                e.field("Total Tracks", &lastfm_user.scrobbles, true);

                // Should be in ISO-8601
                e.timestamp(
                    lastfm_user
                        .registered
                        .date
                        .format("%Y-%m-%dT%H:%M:%S")
                        .to_string(),
                );

                e.footer(|f| {
                    f.text("Registered");

                    f
                });

                e
            });

            m
        })
        .await?;

    Ok(())
}
