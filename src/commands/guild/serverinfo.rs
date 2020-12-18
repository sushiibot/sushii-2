use serenity::framework::standard::{macros::command, CommandResult};
use serenity::model::prelude::*;
use serenity::prelude::*;

#[command]
#[only_in("guild")]
async fn serverinfo(ctx: &Context, msg: &Message) -> CommandResult {
    let guild = match msg.guild(ctx).await {
        Some(id) => id,
        None => {
            msg.channel_id.say(&ctx.http, "No guild found").await?;

            return Ok(());
        }
    };

    let owner = guild.owner_id.to_user(ctx).await?;

    let features_str = if !guild.features.is_empty() {
        guild.features.join(", ")
    } else {
        "None".into()
    };

    msg.channel_id
        .send_message(ctx, |m| {
            m.embed(|e| {
                e.author(|a| {
                    a.name(&guild.name);

                    if let Some(url) = guild.icon_url() {
                        a.url(url);
                    }

                    a
                });

                if let Some(url) = guild.icon_url() {
                    e.thumbnail(url);
                }

                e.field(
                    "Owner",
                    &format!("{} (ID {})", owner.tag(), owner.id.0),
                    true,
                );
                e.field("Features", features_str, true);
                e.field("Channels", &guild.channels.len().to_string(), true);
                e.field("Emojis", &guild.emojis.len().to_string(), true);
                e.field(
                    "Explicit Content Filter",
                    &format!("{:?}", guild.explicit_content_filter),
                    true,
                );
                e.field("Member Count", &guild.member_count.to_string(), true);
                e.field("Region", &guild.region, true);
                e.field("Roles", &guild.roles.len().to_string(), true);
                e.field(
                    "Verification Level",
                    &format!("{:?}", guild.verification_level),
                    true,
                );
                e.field(
                    "Created At",
                    guild.id.created_at().format("%Y-%m-%dT%H:%M:%S"),
                    false,
                );
                e.footer(|f| {
                    f.text(&format!("Guild ID: {}", &guild.id.0));

                    f
                });

                e
            })
        })
        .await?;

    Ok(())
}
