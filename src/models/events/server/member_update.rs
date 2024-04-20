use serde::{Deserialize, Serialize};

use crate::{
    models::{Attachment, Member, MemberID, Server, User, ID},
    Context,
};

use anyhow::Result;

#[cfg(feature = "cache")]
use crate::cache::UpdateCache;

/// Specifies a field to remove on server member update.
#[derive(Debug, Clone, Copy, PartialEq, Deserialize, Serialize)]
pub enum MemberField {
    /// Server member nickname.
    Nickname,
    /// Server member avatar.
    Avatar,
}

/// A server member details were updated.
#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct ServerMemberUpdateEvent {
    /// Member id.
    #[serde(rename = "id")]
    pub member_id: MemberID,
    /// A partial server member.
    pub data: PartialMember,
    /// A specified field to remove on server member update.
    pub clear: Option<MemberField>,
}

impl ServerMemberUpdateEvent {
    /// Fetch the member.
    pub async fn member(&self, ctx: &Context) -> Result<Member> {
        Member::fetch(ctx, &self.member_id.server_id, &self.member_id.user_id).await
    }

    /// Fetch the server.
    pub async fn server(&self, ctx: &Context) -> Result<Server> {
        Server::fetch(ctx, &self.member_id.server_id).await
    }

    /// Fetch the user.
    pub async fn user(&self, ctx: &Context) -> Result<User> {
        User::fetch(ctx, &self.member_id.user_id).await
    }
}

/// A partial server member.
#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct PartialMember {
    /// Member nickname.
    pub nickname: Option<String>,
    /// Member avatar.
    pub avatar: Option<Attachment>,
    /// Member roles.
    #[serde(default)]
    pub roles: Vec<ID>,
}

#[cfg(feature = "cache")]
#[async_trait::async_trait]
impl UpdateCache for ServerMemberUpdateEvent {
    async fn update(&self, ctx: &Context) {
        if let Some(member) = ctx.cache.members.write().await.get_mut(&self.member_id) {
            if let Some(field) = self.clear {
                match field {
                    MemberField::Nickname => member.nickname = None,
                    MemberField::Avatar => member.avatar = None,
                }
            }

            if let Some(ref nickname) = self.data.nickname {
                member.nickname = Some(nickname.clone());
            }

            if let Some(ref avatar) = self.data.avatar {
                member.avatar = Some(avatar.clone());
            }

            if !self.data.roles.is_empty() {
                member.roles = self.data.roles.clone();
            }
        }
    }
}
