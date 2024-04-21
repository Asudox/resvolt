pub use {direct_message::*, group::*, text::*, voice::*};

mod direct_message;
mod group;
mod text;
mod voice;

use serde::Deserialize;

use crate::{builders::EditChannel, models::ID, Context};

use anyhow::Result;

#[cfg(feature = "cache")]
use crate::cache::UpdateCache;

/// A channel.
#[derive(Debug, Deserialize, Clone, PartialEq)]
#[serde(tag = "channel_type")]
pub enum Channel {
    /// A text channel.
    #[serde(rename = "TextChannel")]
    Text(TextChannel),
    /// A voice channel.
    #[serde(rename = "VoiceChannel")]
    Voice(VoiceChannel),
    /// A group channel.
    Group(GroupChannel),
    /// A DM channel.
    DirectMessage(DirectMessageChannel),
}

impl Channel {
    /// Get a channel from the cache or API.
    pub async fn fetch(ctx: &Context, id: &ID) -> Result<Self> {
        #[cfg(feature = "cache")]
        if let Some(channel) = ctx.cache.channel(id).await {
            return Ok(channel);
        }

        ctx.http_client.get(format!("channels/{}", id)).await
    }

    /// Returns the channel id.
    pub fn id(&self) -> &ID {
        match self {
            Self::Text(TextChannel { id, .. })
            | Self::Voice(VoiceChannel { id, .. })
            | Self::Group(GroupChannel { id, .. })
            | Self::DirectMessage(DirectMessageChannel { id, .. }) => id,
        }
    }

    async fn edit(ctx: &Context, channel_id: &ID, builder: EditChannel) -> Result<()> {
        ctx.http_client
            .patch(format!("channels/{}", channel_id), builder)
            .await
    }

    async fn delete(ctx: &Context, channel_id: &ID) -> Result<()> {
        ctx.http_client
            .delete(format!("channels/{}", channel_id))
            .await
    }

    pub fn get_text_channel(self) -> Option<TextChannel>{
        if let Self::Text(TextChannel) = self {
            Some(TextChannel)
        } else {
            None
        }
    }

    pub fn get_voice_channel(self) -> Option<VoiceChannel>{
        if let Self::Voice(VoiceChannel) = self {
            Some(VoiceChannel)
        } else {
            None
        }
    }

    pub fn get_group_channel(self) -> Option<GroupChannel>{
        if let Self::Group(GroupChannel) = self {
            Some(GroupChannel)
        } else {
            None
        }
    }

    pub fn get_dm_channel(self) -> Option<DirectMessageChannel>{
        if let Self::DirectMessage(DirectMessageChannel) = self {
            Some(DirectMessageChannel)
        } else {
            None
        }
    }
}

#[cfg(feature = "cache")]
#[async_trait::async_trait]
impl UpdateCache for Channel {
    async fn update(&self, ctx: &Context) {
        ctx.cache
            .channels
            .write()
            .await
            .insert(self.id().clone(), self.clone());
    }
}
