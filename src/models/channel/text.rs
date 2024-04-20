use serde::Deserialize;

use crate::{
    builders::{CreateMessage, EditChannel},
    models::{Attachment, Channel, Message, ID},
    Context,
};

use anyhow::Result;

/// A text channel.
#[derive(Debug, Deserialize, Clone, PartialEq)]
pub struct TextChannel {
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
    /// ID of last message in the channel.
    pub last_message_id: Option<ID>,
    /// Channel is not safe for work (+18).
    #[serde(default)]
    pub nsfw: bool,
}

impl TextChannel {
    /// Send a message in this channel.
    pub async fn send(&self, ctx: &Context, builder: impl Into<CreateMessage>) -> Result<Message> {
        Message::create(ctx, &self.id, builder.into()).await
    }

    /// Edit the channel.
    pub async fn edit(&self, ctx: &Context, builder: EditChannel) -> Result<()> {
        Channel::edit(ctx, &self.id, builder).await
    }

    /// Delete the channel.
    pub async fn delete(&self, ctx: &Context) -> Result<()> {
        Channel::delete(ctx, &self.id).await
    }
}
