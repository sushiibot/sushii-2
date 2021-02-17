use chrono::Utc;
use heim::cpu::os::unix;
use heim::units::{information, ratio, time};
use heim::{memory, process};
use serenity::framework::standard::{macros::command, CommandResult};
use serenity::model::prelude::*;
use serenity::prelude::*;
use std::time::Duration;

#[command]
async fn ping(ctx: &Context, msg: &Message) -> CommandResult {
    msg.channel_id.say(&ctx.http, "Pong!").await?;

    Ok(())
}

#[command]
#[description("Gets the invite link for sushii")]
async fn invite(ctx: &Context, msg: &Message) -> CommandResult {
    // TODO: Pass invite link via config
    msg.channel_id.say(
        ctx,
        "https://discord.com/oauth2/authorize?client_id=193163942502072320&permissions=268823622&scope=bot"
    ).await?;

    Ok(())
}

#[command]
#[aliases("stats")]
async fn about(ctx: &Context, msg: &Message) -> CommandResult {
    let version = env!("CARGO_PKG_VERSION");

    // Host CPU
    let (one, five, fifteen) = unix::loadavg().await?;

    let proc = process::current().await?;
    let start_time = proc.create_time().await?;

    let now = Utc::now().timestamp();
    let up_time = Duration::from_secs_f64(now as f64 - start_time.get::<time::second>().floor());

    // Process memory
    let mem = proc.memory().await?.rss();
    // Host total memory
    let total_mem = memory::memory().await?.total();

    let _ = msg
        .channel_id
        .send_message(&ctx.http, |m| {
            m.embed(|e| {
                e.title(format!("sushii v{}", version));
                e.color(0xe67e22);

                e.field(
                    "Load Avg",
                    format!(
                        "{}, {}, {}",
                        one.get::<ratio::ratio>(),
                        five.get::<ratio::ratio>(),
                        fifteen.get::<ratio::ratio>(),
                    ),
                    true,
                );
                e.field(
                    "Memory Usage",
                    format!(
                        "{} / {} MB",
                        mem.get::<information::megabyte>(),
                        total_mem.get::<information::megabyte>()
                    ),
                    true,
                );
                e.field("Uptime", humantime::format_duration(up_time), false);

                e
            })
        })
        .await?;

    Ok(())
}
