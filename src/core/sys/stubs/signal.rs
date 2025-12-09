//! Signal processing utilities

use super::{error, sys, traps};

/// A stub enum representing system signals on unsupported platforms.
#[allow(unnameable_types)]
#[allow(clippy::upper_case_acronyms)]
#[derive(Clone, Copy, Eq, Hash, PartialEq, Debug)]
pub enum Signal {
    /// Interrupt signal (Ctrl+C)
    SIGINT,
    /// Termination signal
    SIGTERM,
    /// Kill signal (cannot be caught)
    SIGKILL,
    /// Hangup signal
    SIGHUP,
    /// Quit signal
    SIGQUIT,
    /// User-defined signal 1
    SIGUSR1,
    /// User-defined signal 2
    SIGUSR2,
}

impl Signal {
    /// Returns an iterator over all possible signals.
    pub fn iterator() -> impl Iterator<Item = Self> {
        [
            Signal::SIGINT,
            Signal::SIGTERM,
            Signal::SIGKILL,
            Signal::SIGHUP,
            Signal::SIGQUIT,
            Signal::SIGUSR1,
            Signal::SIGUSR2,
        ]
        .into_iter()
    }

    /// Converts the signal into its corresponding name as a `&'static str`.
    pub const fn as_str(self) -> &'static str {
        match self {
            Signal::SIGINT => "SIGINT",
            Signal::SIGTERM => "SIGTERM",
            Signal::SIGKILL => "SIGKILL",
            Signal::SIGHUP => "SIGHUP",
            Signal::SIGQUIT => "SIGQUIT",
            Signal::SIGUSR1 => "SIGUSR1",
            Signal::SIGUSR2 => "SIGUSR2",
        }
    }

    /// Creates a `Signal` from a string representation.
    pub fn from_str(s: &str) -> Result<Self, error::Error> {
        match s {
            "SIGINT" | "INT" | "2" => Ok(Signal::SIGINT),
            "SIGTERM" | "TERM" | "15" => Ok(Signal::SIGTERM),
            "SIGKILL" | "KILL" | "9" => Ok(Signal::SIGKILL),
            "SIGHUP" | "HUP" | "1" => Ok(Signal::SIGHUP),
            "SIGQUIT" | "QUIT" | "3" => Ok(Signal::SIGQUIT),
            "SIGUSR1" | "USR1" | "10" => Ok(Signal::SIGUSR1),
            "SIGUSR2" | "USR2" | "12" => Ok(Signal::SIGUSR2),
            _ => Err(error::ErrorKind::InvalidSignal(s.into()).into()),
        }
    }

    /// Converts the signal to its raw integer value.
    pub const fn as_raw(self) -> i32 {
        match self {
            Signal::SIGINT => 2,
            Signal::SIGTERM => 15,
            Signal::SIGKILL => 9,
            Signal::SIGHUP => 1,
            Signal::SIGQUIT => 3,
            Signal::SIGUSR1 => 10,
            Signal::SIGUSR2 => 12,
        }
    }
}

impl TryFrom<i32> for Signal {
    type Error = error::Error;

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(Signal::SIGHUP),
            2 => Ok(Signal::SIGINT),
            3 => Ok(Signal::SIGQUIT),
            9 => Ok(Signal::SIGKILL),
            10 => Ok(Signal::SIGUSR1),
            12 => Ok(Signal::SIGUSR2),
            15 => Ok(Signal::SIGTERM),
            _ => Err(error::ErrorKind::InvalidSignal(std::format!("{value}")).into()),
        }
    }
}

pub(crate) fn continue_process(_pid: sys::process::ProcessId) -> Result<(), error::Error> {
    Err(error::ErrorKind::NotSupportedOnThisPlatform("continuing process").into())
}

/// Sends a signal to a specific process.
///
/// This is a stub implementation that returns an error.
pub fn kill_process(
    _pid: sys::process::ProcessId,
    _signal: traps::TrapSignal,
) -> Result<(), error::Error> {
    Err(error::ErrorKind::NotSupportedOnThisPlatform("killing process").into())
}

pub(crate) fn lead_new_process_group() -> Result<(), error::Error> {
    Ok(())
}

pub(crate) struct FakeSignal {}

impl FakeSignal {
    fn new() -> Self {
        Self {}
    }

    pub async fn recv(&self) {
        futures::future::pending::<()>().await;
    }
}

pub(crate) fn tstp_signal_listener() -> Result<FakeSignal, error::Error> {
    Ok(FakeSignal::new())
}

pub(crate) fn chld_signal_listener() -> Result<FakeSignal, error::Error> {
    Ok(FakeSignal::new())
}

pub(crate) async fn await_ctrl_c() -> std::io::Result<()> {
    FakeSignal::new().recv().await;
    Ok(())
}

pub(crate) fn mask_sigttou() -> Result<(), error::Error> {
    Ok(())
}

pub(crate) fn poll_for_stopped_children() -> Result<bool, error::Error> {
    Ok(false)
}
