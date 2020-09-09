use serenity::framework::standard::{macros::command, CommandResult};
use serenity::model::prelude::*;
use serenity::prelude::*;
use std::fmt::Write;

#[command]
async fn listids(ctx: &Context, msg: &Message) -> CommandResult {
    let guild = match msg.guild(&ctx.cache).await {
        Some(g) => g,
        None => {
            msg.channel_id.say(&ctx.http, "No guild found").await?;

            return Ok(());
        }
    };

    let mut roles_text = String::new();

    let mut roles = guild.roles.values().collect::<Vec<&Role>>();
    roles.sort_by(|&a, &b| b.position.cmp(&a.position));

    for role in &roles {
        let _ = write!(
            roles_text,
            "[{:02}] {} │ {}\n",
            role.position, role.id.0, role.name
        );
    }

    // check if over limit, send a text file instead
    if roles_text.len() >= 2000 {
        let files = vec![(roles_text.as_bytes(), "roles.txt")];

        let _ = msg.channel_id.send_files(&ctx.http, files, |m| {
            m.content("Guild role ids are attached in the following text file")
        });
    } else {
        let s = format!("```json\n{}```", roles_text);

        let _ = msg
            .channel_id
            .send_message(&ctx.http, |m| {
                m.embed(|e| {
                    e.title("Server Role IDs");
                    e.description(&s);
                    e.color(0xe67e22);

                    e
                })
            })
            .await?;
    }

    Ok(())
}
