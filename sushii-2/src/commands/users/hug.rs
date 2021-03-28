use rand::{seq::SliceRandom, thread_rng};
use serenity::framework::standard::{macros::command, Args, CommandResult};
use serenity::model::prelude::*;
use serenity::prelude::*;

const HUGS_LEFT: &[&str] = &[
    "ლ(・ヮ・ლ)",
    "⊂(・﹏・⊂)",
    "⊂(・ヮ・⊂)",
    "⊂(・▽・⊂)",
    "ლ(・﹏・ლ)",
    "⊂(･ω･*⊂)",
    "ლ(･ω･*ლ)",
    "ლ(´ ❥ `ლ)",
    "⊂(´・ω・｀⊂)",
];

const HUGS_RIGHT: &[&str] = &[
    "⊂(◉‿◉)つ",
    "(つ◉益◉)つ",
    "(っಠ‿ಠ)っ",
    "ʕっ•ᴥ•ʔっ",
    "(っ・∀・）っ",
    "(っ⇀⑃↼)っ",
    "(つ´∀｀)つ",
    "(つ▀¯▀)つ",
    "(っ´▽｀)っ",
    "(づ￣ ³￣)づ",
    "c⌒っ╹v╹ )っ",
    "(.づ◡﹏◡)づ.",
    "(っ*´∀｀*)っ",
    "(っ⇀`皿′↼)っ",
    "(.づσ▿σ)づ.",
    "<a:aChaelisaHug:695359031782408302>",
];

#[command]
async fn hug(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    let target = args.rest();

    if target.is_empty() {
        msg.channel_id
            .say(&ctx, "Give me someone to hug :(")
            .await?;

        return Ok(());
    }

    // alternate between right and left?
    // thread_rng not send so needs to not have any awaits
    let hug = {
        let mut rng = thread_rng();

        if msg.id.0 % 2 == 0 {
            format!("{} {}", target, HUGS_LEFT.choose(&mut rng).unwrap())
        } else {
            format!("{} {}", HUGS_RIGHT.choose(&mut rng).unwrap(), target)
        }
    };

    msg.channel_id
        .send_message(&ctx, |m| {
            m.content(&hug);

            m.allowed_mentions(|am| {
                am.empty_parse();
                am
            });

            m
        })
        .await?;

    Ok(())
}
