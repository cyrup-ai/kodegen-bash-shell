use crate::interactive::{ReadResult, ShellError};

pub(crate) struct NonTermLineReader;

impl super::LineReader for NonTermLineReader {
    fn read_line(
        &self,
        _prompt: Option<&str>,
        _completion_handler: impl FnMut(
            &str,
            usize,
        ) -> Result<
            crate::core::completion::Completions,
            crate::interactive::ShellError,
        >,
    ) -> Result<crate::interactive::ReadResult, crate::interactive::ShellError> {
        let mut input = String::new();
        let bytes_read = std::io::stdin()
            .read_line(&mut input)
            .map_err(ShellError::InputError)?;

        if bytes_read == 0 {
            Ok(ReadResult::Eof)
        } else {
            Ok(ReadResult::Input(input))
        }
    }
}
