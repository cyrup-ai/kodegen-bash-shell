use clap::{Parser, Subcommand};
use std::io::Write;

use super::events;

pub(crate) fn register(shell: &mut crate::core::Shell) {
    shell.register_builtin(
        "brushctl",
        crate::core::builtins::builtin::<BrushCtlCommand>(),
    );
}

/// Configure the running brush shell.
#[derive(Parser)]
struct BrushCtlCommand {
    #[clap(subcommand)]
    command_group: CommandGroup,
}

#[derive(Subcommand)]
enum CommandGroup {
    #[clap(subcommand)]
    Events(EventsCommand),
}

/// Commands for configuring tracing events.
#[derive(Subcommand)]
enum EventsCommand {
    /// Display status of enabled events.
    Status,

    /// Enable event.
    Enable {
        /// Event to enable.
        event: events::TraceEvent,
    },

    /// Disable event.
    Disable {
        /// Event to disable.
        event: events::TraceEvent,
    },
}

impl crate::core::builtins::Command for BrushCtlCommand {
    type Error = crate::core::Error;

    async fn execute(
        &self,
        context: crate::core::ExecutionContext<'_>,
    ) -> Result<crate::core::ExecutionResult, Self::Error> {
        match self.command_group {
            CommandGroup::Events(ref events) => events.execute(&context),
        }
    }
}

impl EventsCommand {
    fn execute(
        &self,
        context: &crate::core::ExecutionContext<'_>,
    ) -> Result<crate::core::ExecutionResult, crate::core::Error> {
        let event_config = super::entry::get_event_config();

        let mut event_config = event_config.try_lock().map_err(|_| {
            crate::core::Error::from(crate::core::ErrorKind::Unimplemented(
                "Failed to acquire lock on event configuration",
            ))
        })?;

        if let Some(event_config) = event_config.as_mut() {
            match self {
                Self::Status => {
                    let enabled_events = event_config.get_enabled_events();
                    for event in enabled_events {
                        writeln!(context.stdout(), "{event}")?;
                    }
                }
                Self::Enable { event } => event_config.enable(*event)?,
                Self::Disable { event } => event_config.disable(*event)?,
            }

            Ok(crate::core::ExecutionResult::success())
        } else {
            Err(crate::core::ErrorKind::Unimplemented("event configuration not initialized").into())
        }
    }
}
