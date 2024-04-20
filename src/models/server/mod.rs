pub use {category::*, flags::*, member::*, permissions::*, system_message_channels::*};

mod category;
mod flags;
mod member;
mod permissions;
mod system_message_channels;

use serde::Deserialize;

use crate::{
    builders::{CreateChannel, EditServer},
    models::{Attachment, Channel, User, ID},
    Context,
};

use anyhow::Result;

/// A server.
#[derive(Debug, Deserialize, Clone, PartialEq)]
pub struct Server {
    /// Server id.
    #[serde(rename = "_id")]
    pub id: ID,
    /// Server owner id.
    #[serde(rename = "owner")]
    pub owner_id: ID,
    /// Server name.
    pub name: String,
    /// Server description.
    pub description: Option<String>,
    /// Server channels ids.
    pub channels: Vec<ID>,
    /// Server categories.
    #[serde(default)]
    pub categories: Vec<Category>,
    /// Server system message channels.
    #[serde(default)]
    pub system_messages: SystemMessageChannels,
    /// Server icon.
    pub icon: Option<Attachment>,
    /// Server banner.
    pub banner: Option<Attachment>,
    /// Server flags.
    #[serde(default)]
    pub flags: ServerFlags,
    /// Server is not safe for work.
    #[serde(default)]
    pub nsfw: bool,
}

impl Server {
    /// Get a server from the cache or API.
    pub async fn fetch(ctx: &Context, id: &ID) -> Result<Self> {
        #[cfg(feature = "cache")]
        if let Some(server) = ctx.cache.server(id).await {
            return Ok(server);
        }

        ctx.http_client.get(format!("servers/{}", id)).await
    }

    /// Fetch all server members.
    pub async fn members(&self, ctx: &Context) -> Result<Vec<(Member, User)>> {
        let members: ServerMembers = ctx
            .http_client
            .get(format!("servers/{}/members", self.id))
            .await?;

        Ok(members.into())
    }

    /// Edit the server.
    pub async fn edit(&self, ctx: &Context, builder: EditServer) -> Result<()> {
        ctx.http_client
            .patch(format!("servers/{}", self.id), builder)
            .await
    }

    /// Create a channel in the server.
    pub async fn create_channel(&self, ctx: &Context, builder: CreateChannel) -> Result<Channel> {
        ctx.http_client
            .post(format!("servers/{}/channels", self.id), builder)
            .await
    }

    /// Leave the server
    pub async fn leave(&self, ctx: &Context) -> Result<()> {
        ctx.http_client.delete(format!("servers/{}", self.id)).await
    }

    /// Unban a user from the server.
    pub async fn unban(&self, ctx: &Context, user_id: &ID) -> Result<()> {
        ctx.http_client
            .delete(format!("servers/{}/bans/{}", self.id, user_id))
            .await
    }
}

#[derive(Debug, Deserialize)]
struct ServerMembers {
    members: Vec<Member>,
    users: Vec<User>,
}

impl From<ServerMembers> for Vec<(Member, User)> {
    fn from(val: ServerMembers) -> Self {
        val.members.into_iter().zip(val.users).collect()
    }
}
