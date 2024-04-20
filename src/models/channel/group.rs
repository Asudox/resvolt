use serde::Deserialize;

use crate::{
    builders::{CreateMessage, EditChannel},
    models::{Attachment, Channel, Message, User, ID},
    Context,
};

use anyhow::Result;

/// A group channel.
#[derive(Debug, Deserialize, Clone, PartialEq)]
pub struct GroupChannel {
    /// Group id.
    #[serde(rename = "_id")]
    pub id: ID,
    /// Group owner id.
    #[serde(rename = "owner")]
    pub owner_id: ID,
    /// Group name.
    pub name: String,
    /// Group description.
    pub description: Option<String>,
    /// List of user ids who are participating in this group.
    pub recipients: Vec<ID>,
    /// Group icon.
    pub icon: Option<Attachment>,
    /// ID of last message in the group.
    pub last_message_id: Option<ID>,
    /// Group is not safe for work.
    #[serde(default)]
    pub nsfw: bool,
}

impl GroupChannel {
    /// Fetch users who are part of the group.
    pub async fn members(&self, ctx: &Context) -> Result<Vec<User>> {
        ctx.http_client
            .get(format!("channels/{}/members", self.id))
            .await
    }

    /// Send a message in the group.
    pub async fn send(&self, ctx: &Context, builder: impl Into<CreateMessage>) -> Result<Message> {
        Message::create(ctx, &self.id, builder.into()).await
    }

    /// Edit the group.
    pub async fn edit(&self, ctx: &Context, builder: EditChannel) -> Result<()> {
        Channel::edit(ctx, &self.id, builder).await
    }

    /// Leave the group.
    pub async fn leave(&self, ctx: &Context) -> Result<()> {
        Channel::delete(ctx, &self.id).await
    }
}
