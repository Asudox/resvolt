use serde::Deserialize;

use crate::{
    models::{Channel, User, ID},
    Context,
};

use anyhow::Result;

/// A user has started typing in a channel.
#[derive(Debug, Deserialize, Clone, PartialEq)]
pub struct ChannelStartTypingEvent {
    /// Channel id.
    #[serde(rename = "id")]
    pub channel_id: ID,
    /// User id.
    #[serde(rename = "user")]
    pub user_id: ID,
}

impl ChannelStartTypingEvent {
    /// Fetch the channel.
    pub async fn channel(&self, ctx: &Context) -> Result<Channel> {
        Channel::fetch(ctx, &self.channel_id).await
    }

    /// Fetch the user.
    pub async fn user(&self, ctx: &Context) -> Result<User> {
        User::fetch(ctx, &self.user_id).await
    }
}
