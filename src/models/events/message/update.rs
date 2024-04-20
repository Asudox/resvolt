use serde::Deserialize;

use crate::{
    models::{Channel, Embed, Message, MessageEdited, ID},
    Context,
};

use anyhow::Result;

/// A message has been edited or otherwise updated.
#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct MessageUpdateEvent {
    /// Message id.
    #[serde(rename = "id")]
    pub message_id: ID,
    /// Message channel id.
    #[serde(rename = "channel")]
    pub channel_id: ID,
    /// A partial message.
    pub data: PartialMessage,
}

impl MessageUpdateEvent {
    /// Fetch the message.
    pub async fn message(&self, ctx: &Context) -> Result<Message> {
        Message::fetch(ctx, &self.channel_id, &self.message_id).await
    }

    /// Fetch the channel.
    pub async fn channel(&self, ctx: &Context) -> Result<Channel> {
        Channel::fetch(ctx, &self.channel_id).await
    }
}

/// A partial message.
#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct PartialMessage {
    /// Message content.
    pub content: Option<String>,
    /// Message embeds.
    #[serde(default)]
    pub embeds: Vec<Embed>,
    /// Message edition date.
    pub edited: MessageEdited,
}
