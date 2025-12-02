mod basic_shell;
mod non_term_line_reader;
mod term_line_reader;

pub use basic_shell::BasicShell;

use crate::interactive::{ReadResult, ShellError};

pub(crate) trait LineReader {
    fn read_line(
        &self,
        prompt: Option<&str>,
        completion_handler: impl FnMut(
            &str,
            usize,
        )
            -> Result<crate::core::completion::Completions, ShellError>,
    ) -> Result<ReadResult, ShellError>;
}
