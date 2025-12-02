use super::{
    ExecutionParameters, error, expansion,
    shell::Shell,
    sys::{self, users},
};
use std::path::Path;

const VERSION_MAJOR: &str = env!("CARGO_PKG_VERSION_MAJOR");
const VERSION_MINOR: &str = env!("CARGO_PKG_VERSION_MINOR");
const VERSION_PATCH: &str = env!("CARGO_PKG_VERSION_PATCH");

pub(crate) async fn expand_prompt(
    shell: &mut Shell,
    params: &ExecutionParameters,
    spec: String,
) -> Result<String, error::Error> {
    // Parse the prompt spec into its pieces.
    let prompt_pieces = parse_prompt(spec)?;

    // Now, render each piece.
    let mut formatted_prompt = String::new();
    for piece in prompt_pieces {
        let needs_escaping = matches!(
            piece,
            crate::parser::prompt::PromptPiece::EscapedSequence(_)
                | crate::parser::prompt::PromptPiece::DollarOrPound
        );

        let formatted_piece = format_prompt_piece(shell, piece)?;

        if shell.options.expand_prompt_strings && needs_escaping {
            formatted_prompt.push('\\');
        }

        formatted_prompt.push_str(&formatted_piece);
    }

    if shell.options.expand_prompt_strings {
        // Now expand any remaining escape sequences.
        formatted_prompt = expansion::basic_expand_str(shell, params, &formatted_prompt).await?;
    }

    Ok(formatted_prompt)
}

#[cached::proc_macro::cached(size = 64, result = true)]
fn parse_prompt(
    spec: String,
) -> Result<Vec<crate::parser::prompt::PromptPiece>, crate::parser::WordParseError> {
    crate::parser::prompt::parse(spec.as_str())
}

fn format_prompt_piece(
    shell: &Shell,
    piece: crate::parser::prompt::PromptPiece,
) -> Result<String, error::Error> {
    let formatted = match piece {
        crate::parser::prompt::PromptPiece::EscapedSequence(s) => s,
        crate::parser::prompt::PromptPiece::Literal(l) => l,
        crate::parser::prompt::PromptPiece::AsciiCharacter(c) => {
            char::from_u32(c).map_or_else(String::new, |c| c.to_string())
        }
        crate::parser::prompt::PromptPiece::Backslash => "\\".to_owned(),
        crate::parser::prompt::PromptPiece::BellCharacter => "\x07".to_owned(),
        crate::parser::prompt::PromptPiece::CarriageReturn => "\r".to_owned(),
        crate::parser::prompt::PromptPiece::CurrentCommandNumber => {
            return error::unimp("prompt: current command number");
        }
        crate::parser::prompt::PromptPiece::CurrentHistoryNumber => {
            return error::unimp("prompt: current history number");
        }
        crate::parser::prompt::PromptPiece::CurrentUser => users::get_current_username()?,
        crate::parser::prompt::PromptPiece::CurrentWorkingDirectory {
            tilde_replaced,
            basename,
        } => format_current_working_directory(shell, tilde_replaced, basename),
        crate::parser::prompt::PromptPiece::Date(format) => {
            format_date(&chrono::Local::now(), &format)
        }
        crate::parser::prompt::PromptPiece::DollarOrPound => {
            if users::is_root() {
                "#".to_owned()
            } else {
                "$".to_owned()
            }
        }
        crate::parser::prompt::PromptPiece::EndNonPrintingSequence => String::new(),
        crate::parser::prompt::PromptPiece::EscapeCharacter => "\x1b".to_owned(),
        crate::parser::prompt::PromptPiece::Hostname {
            only_up_to_first_dot,
        } => {
            let hn = sys::network::get_hostname()
                .unwrap_or_default()
                .to_string_lossy()
                .to_string();
            if only_up_to_first_dot {
                if let Some((first, _)) = hn.split_once('.') {
                    return Ok(first.to_owned());
                }
            }
            hn
        }
        crate::parser::prompt::PromptPiece::Newline => "\n".to_owned(),
        crate::parser::prompt::PromptPiece::NumberOfManagedJobs => shell.jobs.jobs.len().to_string(),
        crate::parser::prompt::PromptPiece::ShellBaseName => {
            if let Some(shell_name) = &shell.shell_name {
                Path::new(shell_name)
                    .file_name()
                    .map(|name| name.to_string_lossy().to_string())
                    .unwrap_or_default()
            } else {
                String::new()
            }
        }
        crate::parser::prompt::PromptPiece::ShellRelease => {
            std::format!("{VERSION_MAJOR}.{VERSION_MINOR}.{VERSION_PATCH}")
        }
        crate::parser::prompt::PromptPiece::ShellVersion => {
            std::format!("{VERSION_MAJOR}.{VERSION_MINOR}")
        }
        crate::parser::prompt::PromptPiece::StartNonPrintingSequence => String::new(),
        crate::parser::prompt::PromptPiece::TerminalDeviceBaseName => {
            return error::unimp("prompt: terminal device base name");
        }
        crate::parser::prompt::PromptPiece::Time(time_fmt) => {
            format_time(&chrono::Local::now(), &time_fmt)
        }
    };

    Ok(formatted)
}

fn format_current_working_directory(shell: &Shell, tilde_replaced: bool, basename: bool) -> String {
    let mut working_dir_str = shell.working_dir().to_string_lossy().to_string();

    if tilde_replaced {
        working_dir_str = shell.tilde_shorten(working_dir_str);
    }

    if basename {
        if let Some(filename) = Path::new(&working_dir_str).file_name() {
            working_dir_str = filename.to_string_lossy().to_string();
        }
    }

    if cfg!(windows) {
        working_dir_str = working_dir_str.replace('\\', "/");
    }

    working_dir_str
}

fn format_time<Tz: chrono::TimeZone>(
    datetime: &chrono::DateTime<Tz>,
    format: &crate::parser::prompt::PromptTimeFormat,
) -> String
where
    Tz::Offset: std::fmt::Display,
{
    let formatted = match format {
        crate::parser::prompt::PromptTimeFormat::TwelveHourAM => datetime.format("%I:%M %p"),
        crate::parser::prompt::PromptTimeFormat::TwelveHourHHMMSS => datetime.format("%I:%M:%S"),
        crate::parser::prompt::PromptTimeFormat::TwentyFourHourHHMM => datetime.format("%H:%M"),
        crate::parser::prompt::PromptTimeFormat::TwentyFourHourHHMMSS => datetime.format("%H:%M:%S"),
    };

    formatted.to_string()
}

fn format_date<Tz: chrono::TimeZone>(
    datetime: &chrono::DateTime<Tz>,
    format: &crate::parser::prompt::PromptDateFormat,
) -> String
where
    Tz::Offset: std::fmt::Display,
{
    match format {
        crate::parser::prompt::PromptDateFormat::WeekdayMonthDate => {
            datetime.format("%a %b %d").to_string()
        }
        crate::parser::prompt::PromptDateFormat::Custom(fmt) => {
            let fmt_items = chrono::format::StrftimeItems::new(fmt);
            datetime.format_with_items(fmt_items).to_string()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_time() {
        // Create a well-known test date/time.
        let dt = chrono::DateTime::parse_from_rfc3339("2024-12-25T13:34:56.789Z").unwrap();

        assert_eq!(
            format_time(&dt, &crate::parser::prompt::PromptTimeFormat::TwelveHourAM),
            "01:34 PM"
        );

        assert_eq!(
            format_time(
                &dt,
                &crate::parser::prompt::PromptTimeFormat::TwentyFourHourHHMMSS
            ),
            "13:34:56"
        );

        assert_eq!(
            format_time(
                &dt,
                &crate::parser::prompt::PromptTimeFormat::TwelveHourHHMMSS
            ),
            "01:34:56"
        );
    }

    #[test]
    fn test_format_date() {
        // Create a well-known test date/time.
        let dt = chrono::DateTime::parse_from_rfc3339("2024-12-25T12:34:56.789Z").unwrap();

        assert_eq!(
            format_date(
                &dt,
                &crate::parser::prompt::PromptDateFormat::WeekdayMonthDate
            ),
            "Wed Dec 25"
        );

        assert_eq!(
            format_date(
                &dt,
                &crate::parser::prompt::PromptDateFormat::Custom(String::from("%Y-%m-%d"))
            ),
            "2024-12-25"
        );

        assert_eq!(
            format_date(
                &dt,
                &crate::parser::prompt::PromptDateFormat::Custom(String::from(
                    "%Y-%m-%d %H:%M:%S.%f"
                ))
            ),
            "2024-12-25 12:34:56.789000000"
        );
    }
}
