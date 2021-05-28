use crate::error::Result;
use serenity::builder::CreateEmbed;
use serenity::collector::reaction_collector::ReactionAction;
use serenity::model::prelude::*;
use serenity::prelude::*;
use std::collections::HashMap;
use std::time::Duration;

pub struct Confirmation {
    create_embed: Option<Box<dyn FnOnce(&mut CreateEmbed) -> &mut CreateEmbed + Send>>,
    /// Reaction choices with names
    options: Vec<(ReactionType, &'static str)>,
    options_map: HashMap<ReactionType, &'static str>,
    timeout: Duration,
    author: UserId,
}

impl Confirmation {
    pub fn new<F>(author: UserId, f: F) -> Self
    where
        F: FnOnce(&mut CreateEmbed) -> &mut CreateEmbed + 'static + Send,
    {
        Self {
            create_embed: Some(Box::new(f)),
            options: Vec::new(),
            options_map: HashMap::new(),
            // Default 1 minute
            timeout: Duration::from_secs(120),
            author,
        }
    }

    /// Set multiple reaction options, overwrites existing options
    pub fn options(mut self, options: Vec<(ReactionType, &'static str)>) -> Self {
        self.options = options;
        self.options_map = self.options.iter().cloned().collect();
        self
    }

    /// Set the duration for how long to wait for a reaction
    pub fn timeout(mut self, timeout: Duration) -> Self {
        self.timeout = timeout;
        self
    }

    pub async fn await_confirmation(
        &mut self,
        ctx: &Context,
        channel: ChannelId,
    ) -> Result<Option<&'static str>> {
        let msg = channel
            .send_message(ctx, |m| {
                m.embed(|e| {
                    if let Some(f) = self.create_embed.take() {
                        f(e);
                    }

                    e
                });
                m.reactions(self.options.iter().map(|o| o.0.clone()));

                m
            })
            .await?;

        // Create clone to move into filter as it needs 'static lifetime
        let options_map = self.options_map.clone();

        let res = if let Some(conf) = msg
            .await_reaction(ctx)
            .author_id(self.author)
            .filter(move |reaction| options_map.contains_key(&reaction.emoji))
            .timeout(self.timeout)
            .await
        {
            match *conf {
                ReactionAction::Added(ref reaction) => {
                    self.options_map.get(&reaction.emoji).cloned()
                }
                // Default await_reaction should not collect removed reactions by default
                _ => unreachable!(),
            }
        } else {
            None
        };

        // Channel::delete_message bypasses cache
        if let Err(e) = channel.delete_message(ctx, msg.id).await {
            tracing::warn!("Failed to delete confirmation message: {:#?}", e);
        }

        Ok(res)
    }
}
