use serenity::framework::standard::{macros::command, CommandResult};
use serenity::model::prelude::*;
use serenity::prelude::*;

#[command]
async fn ping(ctx: &Context, msg: &Message) -> CommandResult {
    msg.channel_id.say(&ctx.http, "Pong!").await?;

    Ok(())
}

#[command]
#[description("Gets the invite link for sushii")]
async fn invite(ctx: &Context, msg: &Message) -> CommandResult {
    // TODO: Pass invite link via config
    msg.channel_id.say(&ctx.http, "https://discord.com/api/oauth2/authorize?client_id=249784936318369793&permissions=268823622&scope=bot").await?;

    Ok(())
}

#[command]
async fn about(ctx: &Context, msg: &Message) -> CommandResult {
    let version = env!("CARGO_PKG_VERSION");
    let github_run_id = option_env!("GITHUB_RUN_ID");

    let _ = msg
        .channel_id
        .send_message(&ctx.http, |m| {
            m.embed(|e| {
                e.title("sushii 2");
                e.color(0xe67e22);

                e.field("Version", version, true);

                if let Some(id) = github_run_id {
                    e.field("Build ID", id, true);
                }

                e
            })
        })
        .await?;

    Ok(())
}
