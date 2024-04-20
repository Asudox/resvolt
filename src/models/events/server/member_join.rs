use serde::Deserialize;

use crate::{
    models::{Member, Server, User, ID},
    Context,
};

use anyhow::Result;

#[cfg(feature = "cache")]
use crate::cache::UpdateCache;

/// A user has joined the server.
#[derive(Debug, Deserialize, Clone, PartialEq)]
pub struct ServerMemberJoinEvent {
    /// Server id.
    #[serde(rename = "id")]
    pub server_id: ID,
    /// User id.
    #[serde(rename = "user")]
    pub user_id: ID,
}

impl ServerMemberJoinEvent {
    /// Fetch the member.
    pub async fn member(&self, ctx: &Context) -> Result<Member> {
        Member::fetch(ctx, &self.server_id, &self.user_id).await
    }

    /// Fetch the server.
    pub async fn server(&self, ctx: &Context) -> Result<Server> {
        Server::fetch(ctx, &self.server_id).await
    }

    /// Fetch the user.
    pub async fn user(&self, ctx: &Context) -> Result<User> {
        User::fetch(ctx, &self.user_id).await
    }
}

#[cfg(feature = "cache")]
#[async_trait::async_trait]
impl UpdateCache for ServerMemberJoinEvent {
    async fn update(&self, ctx: &Context) {
        if let Ok(member) = Member::fetch(ctx, &self.server_id, &self.user_id).await {
            ctx.cache
                .members
                .write()
                .await
                .insert(member.id.clone(), member);
        }
    }
}
