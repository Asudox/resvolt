use serde::Deserialize;

use crate::{
    models::{Channel, Message, User, ID},
    Context,
};

use anyhow::Result;

/// You have acknowledged new messages in the channel up to the message id.
#[derive(Debug, Deserialize, Clone, PartialEq)]
pub struct ChannelAckEvent {
    /// Channel id.
    #[serde(rename = "id")]
    pub channel_id: ID,
    /// User id.
    #[serde(rename = "user")]
    pub user_id: ID,
    /// Message id.
    pub message_id: ID,
}

impl ChannelAckEvent {
    /// Fetch the channel.
    pub async fn channel(&self, ctx: &Context) -> Result<Channel> {
        Channel::fetch(ctx, &self.channel_id).await
    }

    /// Fetch the user.
    pub async fn user(&self, ctx: &Context) -> Result<User> {
        User::fetch(ctx, &self.user_id).await
    }

    /// Fetch the message.
    pub async fn message(&self, ctx: &Context) -> Result<Message> {
        Message::fetch(ctx, &self.channel_id, &self.message_id).await
    }
}
