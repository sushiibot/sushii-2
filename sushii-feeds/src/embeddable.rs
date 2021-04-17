use anyhow::Result;
use std::time::Duration;
use twilight_embed_builder::{EmbedAuthorBuilder, EmbedBuilder, ImageSource};
use twilight_model::channel::embed::Embed;
use vlive::model::{
    recent_video::RecentVideo as VliveRecentVideo, video::PostDetail as VlivePostDetail,
};

pub trait Embeddable {
    fn to_embed(&self) -> Result<Embed>;
}

impl Embeddable for (VliveRecentVideo, VlivePostDetail) {
    fn to_embed(&self) -> Result<Embed> {
        // Author icon
        let author_icon = if !self.1.author.profile_image_url.is_empty() {
            &self.1.author.profile_image_url
        } else {
            "https://i.imgur.com/NzGrmho.jpg"
        };

        // If live or vod
        let title = if self.0.duration_secs.is_none() {
            format!("[LIVE] {}", self.0.title)
        } else {
            format!("[VOD] {}", self.0.title)
        };

        let description = if let Some(secs) = self.0.duration_secs {
            let d = Duration::from_secs(secs);
            format!("Duration: {}", humantime::format_duration(d))
        } else {
            "".to_string()
        };

        let author = EmbedAuthorBuilder::new()
            .name(&self.0.channel_name)?
            .icon_url(ImageSource::url(author_icon)?)
            .url(self.0.channel_url())
            .build();

        let embed = EmbedBuilder::new()
            .author(author)
            .title(title)?
            .url(self.0.video_url())
            .color(0x1ecfff)?
            .description(description)?
            .image(ImageSource::url(self.0.thumbnail_url())?)
            .build()?;

        Ok(embed)
    }
}
