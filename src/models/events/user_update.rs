use serde::{Deserialize, Serialize};

use crate::{
    models::{Attachment, User, UserProfile, UserStatus, ID},
    Context,
};

use anyhow::Result;

#[cfg(feature = "cache")]
use crate::cache::UpdateCache;

/// Specifies a field to remove on user update.
#[derive(Debug, Clone, Copy, PartialEq, Deserialize, Serialize)]
pub enum UserField {
    /// User profile content.
    ProfileContent,
    /// User profile background.
    ProfileBackground,
    /// User status text.
    StatusText,
    /// User avatar.
    Avatar,
}

/// A user has been updated.
#[derive(Debug, Deserialize, Clone, PartialEq)]
pub struct UserUpdateEvent {
    /// User id.
    #[serde(rename = "id")]
    pub user_id: ID,
    /// A partial user.
    pub data: PartialUser,
    /// A specified field to remove on user update.
    pub clear: Option<UserField>,
}

impl UserUpdateEvent {
    /// Fetch the user.
    pub async fn user(&self, ctx: &Context) -> Result<User> {
        User::fetch(ctx, &self.user_id).await
    }
}

/// A partial user
#[derive(Debug, Deserialize, Clone, PartialEq)]
pub struct PartialUser {
    /// User status.
    pub status: Option<UserStatus>,
    /// User profile.
    pub profile: Option<UserProfile>,
    /// User avatar.
    pub avatar: Option<Attachment>,
    /// Whether the user is online.
    pub online: Option<bool>,
}

#[cfg(feature = "cache")]
#[async_trait::async_trait]
impl UpdateCache for UserUpdateEvent {
    async fn update(&self, ctx: &Context) {
        if let Some(user) = ctx.cache.users.write().await.get_mut(&self.user_id) {
            if let Some(field) = self.clear {
                match field {
                    UserField::StatusText => {
                        if let Some(ref mut status) = user.status {
                            status.text = None;
                        }
                    }
                    UserField::Avatar => user.avatar = None,
                    _ => {}
                }
            }

            if let Some(ref status) = self.data.status {
                user.status = Some(status.clone());
            }

            if let Some(ref avatar) = self.data.avatar {
                user.avatar = Some(avatar.clone());
            }

            if let Some(online) = self.data.online {
                user.online = online;
            }
        }
    }
}
