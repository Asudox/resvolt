use self::command_registry::CommandRegistry;
pub(crate) use action::*;
use tokio::sync::RwLock;

pub use context::Context;
pub use {command::*, error_handler::*, event_handler::*};

mod action;
mod command;
mod command_registry;
mod context;
mod error_handler;
mod event_handler;

// Client struct

use {
    futures_util::{select, FutureExt},
    std::sync::Arc,
    tracing::{error, info, warn},
};

use crate::{
    error::RSError,
    models::{
        events::{ClientEvent, ServerEvent},
        Content,
    },
    state::State,
    websocket::WebSocketClient,
};

use anyhow::Result;

#[cfg(feature = "cache")]
use crate::cache::Cache;

/// API wrapper to interact with Revolt.
pub struct Client<T: RevoltEventHandler> {
    event_handler: Arc<T>,
    ws_client: WebSocketClient,
    action_rx: ActionRx,
    partial_context: Context,
    command_registry: Arc<RwLock<CommandRegistry>>,
    state: Arc<State>,
}

impl<T: RevoltEventHandler> Client<T> {
    /// Create a new client and connect to the server.
    pub async fn new(
        token: impl Into<String>,
        prefix: impl Into<String>,
        event_handler: T,
        state: Option<Arc<State>>,
    ) -> Result<Self> {
        let ws_client = WebSocketClient::connect().await?;
        let (messenger, action_rx) = ActionMessenger::new();
        let partial_context = Context::new(
            token,
            messenger,
            None,
            state.unwrap_or(Arc::new(State::default())),
        )
        .await;

        Ok(Self {
            event_handler: Arc::new(event_handler),
            command_registry: Arc::new(RwLock::new(CommandRegistry::new(prefix.into()))),
            ws_client,
            action_rx,
            partial_context,
            state: Arc::new(State::default()), // state really needed in client struct?
        })
    }

    /// Start listening for server events.
    #[allow(clippy::missing_panics_doc)]
    pub async fn listen(&mut self) -> Result<()> {
        self.authenticate().await?;

        info!(target: "Client", "Client authenticated successfully. Starting listening for events");

        loop {
            if let Err(err) = self.ws_client.check_heartbeat().await {
                warn!(target: "Client", "Err heartbeating: {}", err);
            }

            select! {
                event = self.ws_client.accept().fuse() => {
                    if let Some(event) = event {
                        self.handle_event(event).await;
                    } else {
                        info!(target: "Client", "Connection closed");
                        return Ok(());
                    }
                },
                action = self.action_rx.recv().fuse() => self.handle_action(action.unwrap()).await,
            }
        }
    }

    async fn authenticate(&mut self) -> Result<()> {
        self.ws_client
            .send(ClientEvent::Authenticate {
                token: self.partial_context.token(),
            })
            .await?;

        let event = self.ws_client.accept().await.ok_or(RSError::Unknown(
            "The server closed the connection unexpectedly".into(),
        ))??;

        match event {
            ServerEvent::Authenticated => Ok(()),
            ServerEvent::Error { error } => Err(error.into()),
            event => Err(RSError::Unknown(format!(
                "Unexpected event received while authenticating: {:?}",
                event
            ))
            .into()),
        }
    }

    async fn handle_event(&self, event: Result<ServerEvent>) {
        match event {
            Ok(event) => {
                let event_handler = self.event_handler.clone();
                let command_registry = self.command_registry.clone();
                let partial_ctx = self.partial_context.clone();
                let command_prefix = self.command_registry.read().await.prefix.clone();
                let state = self.state.clone();

                tokio::spawn(async move {
                    #[cfg(feature = "cache")]
                    Cache::update(&partial_ctx, &event).await;

                    if let ServerEvent::Message(ref msg) = event {
                        if let Content::Text(content) = &msg.content {
                            let words: Vec<&str> = content.split_whitespace().collect();
                            if words[0].starts_with(&command_prefix) {
                                let ctx = Context::new(
                                    partial_ctx.token(),
                                    partial_ctx.messenger(),
                                    Some(msg.clone()),
                                    state.clone(),
                                )
                                .await;
                                if let Err(err) = command_registry
                                    .read()
                                    .await
                                    .execute_command(
                                        words[0].strip_prefix(&command_prefix).unwrap(),
                                        &ctx,
                                    )
                                    .await
                                {
                                    command_registry
                                        .read()
                                        .await
                                        .handle_error(&ctx, err)
                                        .await
                                        .ok();
                                }
                            } else if let Err(err) = event_handler.handle(&partial_ctx, event).await
                            {
                                command_registry
                                    .read()
                                    .await
                                    .handle_error(&partial_ctx, err)
                                    .await
                                    .ok();
                            }
                        }
                    } else if let Err(err) = event_handler.handle(&partial_ctx, event).await {
                        command_registry
                            .read()
                            .await
                            .handle_error(&partial_ctx, err)
                            .await
                            .ok();
                    }
                });
            }
            Err(err) => error!(target: "Client", "Err handling event: {}", err),
        }
    }

    async fn handle_action(&mut self, action: Action) {
        match action {
            Action::SendEvent { event, tx } => tx.send(self.ws_client.send(event).await).unwrap(),
            Action::GetLatency { tx } => tx.send(self.ws_client.latency()).unwrap(),
            Action::Close { tx } => tx.send(self.ws_client.close().await).unwrap(),
        }
    }

    /// Registers a new [`RevoltCommand`] and its name to the internal command registry
    pub async fn register_command(
        &mut self,
        command_name: impl Into<String>,
        command: impl RevoltCommand,
    ) {
        self.command_registry
            .write()
            .await
            .register_command(command_name.into(), command);
    }

    /// Sets the specified [`RevoltErrorHandler`] in the internal command registry
    pub async fn set_error_handler(&mut self, error_handler: impl RevoltErrorHandler) {
        self.command_registry
            .write()
            .await
            .set_error_handler(error_handler);
    }
}
