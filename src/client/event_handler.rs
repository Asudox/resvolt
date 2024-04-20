use anyhow::Result;
use async_trait::async_trait;

use crate::{
    models::{events::*, *},
    Context,
};

/// Define handlers for supported events.
#[async_trait]
pub trait RevoltEventHandler: Send + Sync + 'static {
    /// Bot is ready.
    async fn on_ready(&self, _ctx: &Context, _data: ReadyEvent) -> Result<()> {
        Ok(())
    }

    /// A new message received.
    async fn on_message(&self, _ctx: &Context, _data: Message) -> Result<()> {
        Ok(())
    }

    /// A message has been edited or otherwise updated.
    async fn on_message_update(&self, _ctx: &Context, _data: MessageUpdateEvent) -> Result<()> {
        Ok(())
    }

    /// A message has been deleted.
    async fn on_message_delete(&self, _ctx: &Context, _data: MessageDeleteEvent) -> Result<()> {
        Ok(())
    }

    /// A channel has been created.
    async fn on_channel_create(&self, _ctx: &Context, _data: Channel) -> Result<()> {
        Ok(())
    }

    /// A channel details were updated.
    async fn on_channel_update(&self, _ctx: &Context, _data: ChannelUpdateEvent) -> Result<()> {
        Ok(())
    }

    /// A channel has been deleted.
    async fn on_channel_delete(&self, _ctx: &Context, _data: ChannelDeleteEvent) -> Result<()> {
        Ok(())
    }

    /// A user has joined the group.
    async fn on_channel_group_join(
        &self,
        _ctx: &Context,
        _data: ChannelGroupJoinEvent,
    ) -> Result<()> {
        Ok(())
    }

    /// A user has left the group.
    async fn on_channel_group_leave(
        &self,
        _ctx: &Context,
        _data: ChannelGroupLeaveEvent,
    ) -> Result<()> {
        Ok(())
    }

    /// A user has started typing in a channel.
    async fn on_channel_start_typing(
        &self,
        _ctx: &Context,
        _data: ChannelStartTypingEvent,
    ) -> Result<()> {
        Ok(())
    }

    /// A user has stopped typing in a channel.
    async fn on_channel_stop_typing(
        &self,
        _ctx: &Context,
        _data: ChannelStopTypingEvent,
    ) -> Result<()> {
        Ok(())
    }

    /// You have acknowledged new messages in the channel up to the message id.
    async fn on_channel_ack(&self, _ctx: &Context, _data: ChannelAckEvent) -> Result<()> {
        Ok(())
    }

    /// A server details were updated.
    async fn on_server_update(&self, _ctx: &Context, _data: ServerUpdateEvent) -> Result<()> {
        Ok(())
    }

    /// A server has been deleted.
    async fn on_server_delete(&self, _ctx: &Context, _data: ServerDeleteEvent) -> Result<()> {
        Ok(())
    }

    /// A server member details were updated.
    async fn on_server_member_update(
        &self,
        _ctx: &Context,
        _data: ServerMemberUpdateEvent,
    ) -> Result<()> {
        Ok(())
    }

    /// A user has joined the group.
    async fn on_server_member_join(
        &self,
        _ctx: &Context,
        _data: ServerMemberJoinEvent,
    ) -> Result<()> {
        Ok(())
    }

    /// A user has left the group.
    async fn on_server_member_leave(
        &self,
        _ctx: &Context,
        _data: ServerMemberLeaveEvent,
    ) -> Result<()> {
        Ok(())
    }

    /// A server role details were updated.
    async fn on_server_role_update(
        &self,
        _ctx: &Context,
        _data: ServerRoleUpdateEvent,
    ) -> Result<()> {
        Ok(())
    }

    /// A server role has been deleted.
    async fn on_server_role_delete(
        &self,
        _ctx: &Context,
        _data: ServerRoleDeleteEvent,
    ) -> Result<()> {
        Ok(())
    }

    /// A user has been updated.
    async fn on_user_update(&self, _ctx: &Context, _data: UserUpdateEvent) -> Result<()> {
        Ok(())
    }
}

#[async_trait]
pub(crate) trait RevoltEventHandlerExt: RevoltEventHandler {
    async fn handle(&self, ctx: &Context, event: ServerEvent) -> Result<()> {
        match event {
            ServerEvent::Ready(data) => self.on_ready(ctx, data).await,
            ServerEvent::Message(msg) => loop {
                // Ignore messages that belong to current bot
                if ctx.http_client.bot.id != msg.author_id {
                    return self.on_message(ctx, msg).await;
                }
            },
            ServerEvent::MessageUpdate(data) => self.on_message_update(ctx, data).await,
            ServerEvent::MessageDelete(data) => self.on_message_delete(ctx, data).await,
            ServerEvent::ChannelCreate(channel) => self.on_channel_create(ctx, channel).await,
            ServerEvent::ChannelUpdate(data) => self.on_channel_update(ctx, data).await,
            ServerEvent::ChannelDelete(data) => self.on_channel_delete(ctx, data).await,
            ServerEvent::ChannelGroupJoin(data) => self.on_channel_group_join(ctx, data).await,
            ServerEvent::ChannelGroupLeave(data) => self.on_channel_group_leave(ctx, data).await,
            ServerEvent::ChannelStartTyping(data) => self.on_channel_start_typing(ctx, data).await,
            ServerEvent::ChannelStopTyping(data) => self.on_channel_stop_typing(ctx, data).await,
            ServerEvent::ChannelAck(data) => self.on_channel_ack(ctx, data).await,
            ServerEvent::ServerUpdate(data) => self.on_server_update(ctx, data).await,
            ServerEvent::ServerDelete(data) => self.on_server_delete(ctx, data).await,
            ServerEvent::ServerMemberUpdate(data) => self.on_server_member_update(ctx, data).await,
            ServerEvent::ServerMemberJoin(data) => self.on_server_member_join(ctx, data).await,
            ServerEvent::ServerMemberLeave(data) => self.on_server_member_leave(ctx, data).await,
            ServerEvent::ServerRoleUpdate(data) => self.on_server_role_update(ctx, data).await,
            ServerEvent::ServerRoleDelete(data) => self.on_server_role_delete(ctx, data).await,
            ServerEvent::UserUpdate(data) => self.on_user_update(ctx, data).await,
            _ => Ok(()),
        }
    }
}

impl<T: RevoltEventHandler> RevoltEventHandlerExt for T {}
