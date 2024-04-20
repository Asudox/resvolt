use serde::Deserialize;

use crate::{
    builders::CreateMessage,
    models::{Channel, Message, ID},
    Context,
};

use anyhow::Result;

/// A DM channel.
#[derive(Debug, Deserialize, Clone, PartialEq)]
pub struct DirectMessageChannel {
    /// Channel id.
    #[serde(rename = "_id")]
    pub id: ID,
    /// Whether this DM is active.
    pub active: bool,
    /// List of user ids who are participating in this DM.
    pub recipients: [ID; 2],
    /// ID of the last message in the channel.
    pub last_message_id: Option<ID>,
}

impl DirectMessageChannel {
    /// Open a DM with another user.
    pub async fn open(ctx: &Context, user_id: &ID) -> Result<Self> {
        ctx.http_client.get(format!("users/{}/dm", user_id)).await
    }

    /// Send a message in this channel.
    pub async fn send(&self, ctx: &Context, builder: impl Into<CreateMessage>) -> Result<Message> {
        Message::create(ctx, &self.id, builder.into()).await
    }

    /// Close the DM.
    pub async fn close(&self, ctx: &Context) -> Result<()> {
        Channel::delete(ctx, &self.id).await
    }
}
