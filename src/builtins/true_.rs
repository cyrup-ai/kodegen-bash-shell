use clap::Parser;

use crate::core::{ExecutionResult, builtins};

/// Return 0.
#[derive(Parser)]
pub(crate) struct TrueCommand {}

impl builtins::Command for TrueCommand {
    type Error = crate::core::Error;

    async fn execute(
        &self,
        _context: crate::core::ExecutionContext<'_>,
    ) -> Result<crate::core::ExecutionResult, Self::Error> {
        Ok(ExecutionResult::success())
    }
}
