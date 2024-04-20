use serde::{Deserialize, Serialize};

use crate::{
    builders::EditMember,
    models::{Attachment, ID},
    Context,
};

use anyhow::Result;

/// A server member id.
#[derive(Debug, Clone, PartialEq, Deserialize, Hash, Eq)]
pub struct MemberID {
    /// Server id.
    #[serde(rename = "server")]
    pub server_id: ID,
    /// User id.
    #[serde(rename = "user")]
    pub user_id: ID,
}

impl From<(&ID, &ID)> for MemberID {
    fn from((server_id, user_id): (&ID, &ID)) -> Self {
        Self {
            server_id: server_id.clone(),
            user_id: user_id.clone(),
        }
    }
}

/// A server member.
#[derive(Debug, Deserialize, Clone, PartialEq)]
pub struct Member {
    /// Member id.
    #[serde(rename = "_id")]
    pub id: MemberID,
    /// Member nickname.
    pub nickname: Option<String>,
    /// Member avatar.
    pub avatar: Option<Attachment>,
    /// Member roles ids.
    #[serde(default)]
    pub roles: Vec<ID>,
}

impl Member {
    /// Get a member from the cache or API.
    pub async fn fetch(ctx: &Context, server_id: &ID, user_id: &ID) -> Result<Self> {
        #[cfg(feature = "cache")]
        if let Some(member) = ctx.cache.member(&(server_id, user_id).into()).await {
            return Ok(member);
        }

        ctx.http_client
            .get(format!("servers/{}/members/{}", server_id, user_id))
            .await
    }

    /// Edit the member.
    pub async fn edit(&self, ctx: &Context, builder: EditMember) -> Result<()> {
        ctx.http_client
            .patch(
                format!("servers/{}/members/{}", self.id.server_id, self.id.user_id),
                builder,
            )
            .await
    }

    /// Kick the member from the server.
    pub async fn kick(&self, ctx: &Context) -> Result<()> {
        ctx.http_client
            .delete(format!(
                "servers/{}/members/{}",
                self.id.server_id, self.id.user_id
            ))
            .await
    }

    /// Ban the member from the server.
    pub async fn ban(&self, ctx: &Context, reason: Option<impl Into<String>>) -> Result<()> {
        ctx.http_client
            .put(
                format!("servers/{}/bans/{}", self.id.server_id, self.id.user_id),
                CreateBan::new(reason),
            )
            .await
    }
}

#[derive(Debug, Serialize)]
struct CreateBan {
    #[serde(skip_serializing_if = "Option::is_none")]
    reason: Option<String>,
}

impl CreateBan {
    fn new(reason: Option<impl Into<String>>) -> Self {
        match reason {
            Some(reason) => CreateBan {
                reason: Some(reason.into()),
            },
            None => CreateBan { reason: None },
        }
    }
}
