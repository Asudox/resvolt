use serde::Deserialize;

use crate::{
    models::{Channel, ID},
    Context,
};

use anyhow::Result;

/// A message has been deleted.
#[derive(Debug, Deserialize, Clone, PartialEq)]
pub struct MessageDeleteEvent {
    /// Message id.
    #[serde(rename = "id")]
    pub message_id: ID,
    /// Message channel id.
    #[serde(rename = "channel")]
    pub channel_id: ID,
}

impl MessageDeleteEvent {
    /// Fetch the channel.
    pub async fn channel(&self, ctx: &Context) -> Result<Channel> {
        Channel::fetch(ctx, &self.channel_id).await
    }
}
