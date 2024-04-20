pub use {badges::*, bot_information::*, flags::*, profile::*, status::*};

mod badges;
mod bot_information;
mod flags;
mod profile;
mod status;

use serde::Deserialize;

use crate::{
    models::{Attachment, DirectMessageChannel, ID},
    Context,
};

use anyhow::Result;

/// A user.
#[derive(Debug, Deserialize, Clone, PartialEq)]
pub struct User {
    /// User id.
    #[serde(rename = "_id")]
    pub id: ID,
    /// User username.
    pub username: String,
    /// User avatar.
    pub avatar: Option<Attachment>,
    /// User status.
    pub status: Option<UserStatus>,
    /// User badges.
    #[serde(default)]
    pub badges: u32,
    /// User flags.
    #[serde(default)]
    pub flags: u8,
    /// User is online.
    #[serde(default)]
    pub online: bool,
    /// The bot information.
    pub bot: Option<BotInformation>,
}

impl User {
    /// Get a user from the cache or API.
    pub async fn fetch(ctx: &Context, id: &ID) -> Result<Self> {
        #[cfg(feature = "cache")]
        if let Some(user) = ctx.cache.user(id).await {
            return Ok(user);
        }

        ctx.http_client.get(format!("users/{}", id)).await
    }

    /// Returns if the user is a bot.
    pub fn is_bot(&self) -> bool {
        self.bot.is_some()
    }

    /// Get the user profile from the API.
    pub async fn profile(&self, ctx: &Context) -> Result<UserProfile> {
        ctx.http_client
            .get(format!("users/{}/profile", self.id))
            .await
    }

    /// Open a DM with the user.
    pub async fn dm(&self, ctx: &Context) -> Result<DirectMessageChannel> {
        DirectMessageChannel::open(ctx, &self.id).await
    }
}
