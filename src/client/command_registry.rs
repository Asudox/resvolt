use crate::{error::RSError, Context, RevoltCommand, RevoltErrorHandler};
use ahash::AHashMap;
use anyhow::Result;

/// A command registry
pub(crate) struct CommandRegistry {
    pub(crate) prefix: String,
    commands: AHashMap<String, Box<dyn RevoltCommand>>,
    error_handler: Option<Box<dyn RevoltErrorHandler>>,
}

impl CommandRegistry {
    /// Creates a new [`CommandRegistry`]
    pub(crate) fn new(prefix: String) -> Self {
        Self {
            prefix,
            commands: AHashMap::new(),
            error_handler: None,
        }
    }

    /// Registers a new [`RevoltCommand`] and its name to the command registry
    pub(crate) fn register_command(&mut self, command_name: String, command: impl RevoltCommand) {
        self.commands.insert(command_name, Box::new(command));
    }

    /// Sets the specified [`RevoltErrorHandler`] in the command registry
    pub(crate) fn set_error_handler(&mut self, error_handler: impl RevoltErrorHandler) {
        self.error_handler = Some(Box::new(error_handler));
    }

    /// Executes a specified command by its specified name
    pub(crate) async fn execute_command(&self, command_name: &str, ctx: &Context) -> Result<()> {
        if let Some(command) = self.commands.get(command_name) {
            command.execute(ctx).await
        } else {
            self.handle_error(
                ctx,
                RSError::CommandNotFound(format!("Command '{}' not found", command_name)).into(),
            )
            .await
            .ok();

            Ok(())
        }
    }

    pub(crate) async fn handle_error(&self, ctx: &Context, error: anyhow::Error) -> Result<()> {
        if let Some(error_handler) = &self.error_handler {
            error_handler.handle(ctx, error).await;

            Ok(())
        } else {
            Err(anyhow::Error::new(RSError::ErrorHandlerNotSet))
        }
    }
}
