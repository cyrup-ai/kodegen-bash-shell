use clap::Parser;

use crate::core::{ExecutionResult, builtins};

/// Pop a path from the current directory stack.
#[derive(Parser)]
pub(crate) struct PopdCommand {
    /// Pop the path without changing the current working directory.
    #[clap(short = 'n')]
    no_directory_change: bool,
    //
    // TODO: implement +N and -N
}

impl builtins::Command for PopdCommand {
    type Error = super::dirs::DirError;

    async fn execute(
        &self,
        context: crate::core::ExecutionContext<'_>,
    ) -> Result<crate::core::ExecutionResult, Self::Error> {
        if let Some(popped) = context.shell.directory_stack.pop() {
            if !self.no_directory_change {
                context.shell.set_working_dir(&popped)?;
            }

            // Display dirs.
            let dirs_cmd = super::dirs::DirsCommand::default();
            dirs_cmd.execute(context).await?;

            Ok(ExecutionResult::success())
        } else {
            Err(super::dirs::DirError::DirStackEmpty)
        }
    }
}
