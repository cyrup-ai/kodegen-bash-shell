use crate::core::{ExecutionExitCode, builtins, trace_categories};

use clap::Parser;

/// (UNIMPLEMENTED COMMAND)
#[derive(Parser)]
pub(crate) struct UnimplementedCommand {
    #[clap(allow_hyphen_values = true)]
    args: Vec<String>,

    #[clap(skip)]
    declarations: Vec<crate::core::CommandArg>,
}

impl builtins::Command for UnimplementedCommand {
    type Error = crate::core::Error;

    async fn execute(
        &self,
        context: crate::core::ExecutionContext<'_>,
    ) -> Result<crate::core::ExecutionResult, Self::Error> {
        tracing::warn!(target: trace_categories::UNIMPLEMENTED,
            "unimplemented built-in: {} {}",
            context.command_name,
            self.args.join(" ")
        );
        Ok(ExecutionExitCode::Unimplemented.into())
    }
}

impl builtins::DeclarationCommand for UnimplementedCommand {
    fn set_declarations(&mut self, declarations: Vec<crate::core::CommandArg>) {
        self.declarations = declarations;
    }
}
