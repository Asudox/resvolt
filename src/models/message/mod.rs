use std::time::Duration;

pub use {content::*, edited::*, embed::*, masquerade::*};

mod content;
mod edited;
mod embed;
mod masquerade;

use serde::Deserialize;

use crate::{
    builders::{CreateMessage, EditMessage},
    models::{Attachment, ID},
    Context,
};

use anyhow::Result;

#[cfg(feature = "cache")]
use crate::{cache::UpdateCache, models::Channel};

/// A message.
#[derive(Debug, Deserialize, Clone, PartialEq)]
pub struct Message {
    /// Message id.
    #[serde(rename = "_id")]
    pub id: ID,
    /// Message nonce.
    pub nonce: Option<String>,
    /// Message channel id.
    #[serde(rename = "channel")]
    pub channel_id: ID,
    /// Message author id.
    #[serde(rename = "author")]
    pub author_id: ID,
    /// Message content.
    pub content: Content,
    /// Message attachments.
    #[serde(default)]
    pub attachments: Vec<Attachment>,
    /// Message embeds.
    #[serde(default)]
    pub embeds: Vec<Embed>,
    /// Message mentions.
    #[serde(default)]
    pub mentions: Vec<ID>,
    /// Message replies.
    #[serde(default)]
    pub replies: Vec<ID>,
    /// Message masquerade.
    pub masquerade: Option<Masquerade>,
    /// Edition date.
    pub edited: Option<MessageEdited>,
}

impl Message {
    /// Get a message from the API.
    pub async fn fetch(ctx: &Context, channel_id: &ID, id: &ID) -> Result<Self> {
        let path = format!("channels/{}/messages/{}", channel_id, id);
        let msg = ctx.http_client.get(&path).await?;

        Ok(msg)
    }

    /// Creates a new [`Message`] in the specified channel ID.
    pub async fn create(ctx: &Context, channel_id: &ID, builder: CreateMessage) -> Result<Self> {
        let path = format!("channels/{}/messages", channel_id);
        let msg = ctx.http_client.post(&path, builder).await?;

        Ok(msg)
    }

    /// Returns whether the message has been edited.
    pub fn is_edited(&self) -> bool {
        self.edited.is_some()
    }

    /// Convenience method that calls the associated [`create`](Self::create) function with the channel ID taken from self.
    ///
    /// Basically creates a new [`Message`] in the same channel as self.
    pub async fn create_from_self(
        &self,
        ctx: &Context,
        builder: impl Into<CreateMessage>,
    ) -> Result<Self> {
        Self::create(ctx, &self.channel_id, builder.into()).await
    }

    /// Reply to the message.
    pub async fn reply(
        &self,
        ctx: &Context,
        builder: impl Into<CreateMessage>,
        mention: bool,
    ) -> Result<Self> {
        Self::create(
            ctx,
            &self.channel_id,
            builder.into().reply(&self.id, mention),
        )
        .await
    }

    /// Edit the message.
    pub async fn edit(&mut self, ctx: &Context, builder: impl Into<EditMessage>) -> Result<()> {
        // TODO: Update local message.
        let path = format!("channels/{}/messages/{}", self.channel_id, self.id);
        ctx.http_client.patch(&path, builder.into()).await?;

        Ok(())
    }

    /// Delete the message.
    pub async fn delete(&self, ctx: &Context) -> Result<()> {
        let path = format!("channels/{}/messages/{}", self.channel_id, self.id);
        ctx.http_client.delete(&path).await?;

        Ok(())
    }

    /// Delete the message after the specified delay is over.
    pub async fn delete_after(&self, ctx: &Context, delay: Duration) -> Result<()> {
        tokio::time::sleep(delay).await;

        let path = format!("channels/{}/messages/{}", self.channel_id, self.id);
        ctx.http_client.delete(&path).await?;

        Ok(())
    }
}

#[cfg(feature = "cache")]
#[async_trait::async_trait]
impl UpdateCache for Message {
    async fn update(&self, ctx: &Context) {
        if let Some(channel) = ctx.cache.channels.write().await.get_mut(&self.channel_id) {
            match channel {
                Channel::Text(channel) => channel.last_message_id = Some(self.id.clone()),
                Channel::Group(channel) => channel.last_message_id = Some(self.id.clone()),
                Channel::DirectMessage(channel) => channel.last_message_id = Some(self.id.clone()),
                _ => {}
            }
        }
    }
}
