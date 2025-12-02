use clap::Parser;
use std::io::Write;

use crate::core::{ExecutionExitCode, ExecutionResult, builtins};

/// Suspend the shell.
#[derive(Parser)]
pub(crate) struct SuspendCommand {
    /// Force suspend login shells.
    #[arg(short = 'f')]
    force: bool,
}

impl builtins::Command for SuspendCommand {
    type Error = crate::core::Error;

    async fn execute(
        &self,
        context: crate::core::ExecutionContext<'_>,
    ) -> Result<ExecutionResult, Self::Error> {
        if context.shell.options.login_shell && !self.force {
            writeln!(context.stderr(), "login shell cannot be suspended")?;
            return Ok(ExecutionExitCode::InvalidUsage.into());
        }

        #[expect(clippy::cast_possible_wrap)]
        crate::core::sys::signal::kill_process(
            std::process::id() as i32,
            crate::core::traps::TrapSignal::Signal(nix::sys::signal::SIGSTOP),
        )?;

        Ok(ExecutionResult::success())
    }
}
