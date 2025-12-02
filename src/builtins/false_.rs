use clap::Parser;

use crate::core::{ExecutionResult, builtins};

/// Return a non-zero exit code.
#[derive(Parser)]
pub(crate) struct FalseCommand {}

impl builtins::Command for FalseCommand {
    type Error = crate::core::Error;

    async fn execute(
        &self,
        _context: crate::core::ExecutionContext<'_>,
    ) -> Result<ExecutionResult, Self::Error> {
        Ok(ExecutionResult::general_error())
    }
}
