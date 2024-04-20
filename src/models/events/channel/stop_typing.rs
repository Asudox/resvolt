use serde::Deserialize;

use crate::{
    models::{Channel, User, ID},
    Context,
};

use anyhow::Result;

/// A user has stopped typing in a channel.
#[derive(Debug, Deserialize, Clone, PartialEq)]
pub struct ChannelStopTypingEvent {
    /// Channel id.
    #[serde(rename = "id")]
    pub channel_id: ID,
    /// User id.
    #[serde(rename = "user")]
    pub user_id: ID,
}

impl ChannelStopTypingEvent {
    /// Fetch the channel.
    pub async fn channel(&self, ctx: &Context) -> Result<Channel> {
        Channel::fetch(ctx, &self.channel_id).await
    }

    /// Fetch the user.
    pub async fn user(&self, ctx: &Context) -> Result<User> {
        User::fetch(ctx, &self.user_id).await
    }
}
