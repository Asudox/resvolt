use serde::Deserialize;

use crate::{
    builders::EditChannel,
    models::{Attachment, Channel, ID},
    Context,
};

use anyhow::Result;

/// A voice channel.
#[derive(Debug, Deserialize, Clone, PartialEq)]
pub struct VoiceChannel {
    /// Channel id.
    #[serde(rename = "_id")]
    pub id: ID,
    /// Channel server id.
    #[serde(rename = "server")]
    pub server_id: ID,
    /// Channel name.
    pub name: String,
    /// Channel description.
    pub description: Option<String>,
    /// Channel icon.
    pub icon: Option<Attachment>,
    /// Channel is not safe for work.
    #[serde(default)]
    pub nsfw: bool,
}

impl VoiceChannel {
    /// Edit the channel.
    pub async fn edit(&self, ctx: &Context, builder: EditChannel) -> Result<()> {
        Channel::edit(ctx, &self.id, builder).await
    }

    /// Delete the channel.
    pub async fn delete(&self, ctx: &Context) -> Result<()> {
        Channel::delete(ctx, &self.id).await
    }
}
