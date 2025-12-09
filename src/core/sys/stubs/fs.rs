//! Filesystem utilities.
//!
//! Provides cross-platform filesystem operations for Windows and other non-Unix platforms.

use std::fs;

use super::error;

impl crate::core::sys::fs::PathExt for std::path::Path {
    fn readable(&self) -> bool {
        // On Windows, if we can get metadata, we can likely read the file
        fs::metadata(self).is_ok()
    }

    fn writable(&self) -> bool {
        // Check if the file exists and is not read-only
        fs::metadata(self)
            .map(|m| !m.permissions().readonly())
            .unwrap_or(false)
    }

    fn executable(&self) -> bool {
        // On Windows, executability is determined by file extension
        // Check common executable extensions
        if !self.exists() {
            return false;
        }
        self.extension()
            .map(|ext| {
                let ext = ext.to_string_lossy().to_lowercase();
                matches!(ext.as_str(), "exe" | "cmd" | "bat" | "com" | "ps1" | "msc")
            })
            .unwrap_or(false)
    }

    fn exists_and_is_block_device(&self) -> bool {
        // Windows doesn't have block devices in the Unix sense
        false
    }

    fn exists_and_is_char_device(&self) -> bool {
        // Windows doesn't have character devices in the Unix sense
        // (CON, NUL, etc. are special but not exposed this way)
        false
    }

    fn exists_and_is_fifo(&self) -> bool {
        // Windows doesn't have FIFOs (named pipes exist but are different)
        false
    }

    fn exists_and_is_socket(&self) -> bool {
        // Windows doesn't have Unix domain sockets in the filesystem
        false
    }

    fn exists_and_is_setgid(&self) -> bool {
        // Windows doesn't have setgid bit
        false
    }

    fn exists_and_is_setuid(&self) -> bool {
        // Windows doesn't have setuid bit
        false
    }

    fn exists_and_is_sticky_bit(&self) -> bool {
        // Windows doesn't have sticky bit
        false
    }

    fn get_device_and_inode(&self) -> Result<(u64, u64), crate::core::error::Error> {
        // Windows has file IDs but they work differently
        // For now, return placeholder values
        // TODO: Use GetFileInformationByHandle for proper file ID
        Ok((0, 0))
    }
}

pub(crate) trait MetadataExt {
    fn gid(&self) -> u32 {
        // Windows doesn't have Unix GID
        0
    }

    fn uid(&self) -> u32 {
        // Windows doesn't have Unix UID
        0
    }
}

impl MetadataExt for fs::Metadata {}

/// Returns the executable search paths from the PATH environment variable.
pub(crate) fn get_default_executable_search_paths() -> Vec<String> {
    std::env::var("PATH")
        .unwrap_or_default()
        .split(';') // Windows uses semicolon as path separator
        .filter(|s| !s.is_empty())
        .map(String::from)
        .collect()
}

/// Returns the default paths where standard utilities are typically installed on Windows.
pub fn get_default_standard_utils_paths() -> Vec<String> {
    // Windows doesn't have standard Unix utility paths
    // System utilities are typically in System32
    let system_root = std::env::var("SystemRoot").unwrap_or_else(|_| "C:\\Windows".to_string());
    vec![
        format!("{}\\System32", system_root),
        format!("{}\\System32\\WindowsPowerShell\\v1.0", system_root),
    ]
}

/// Opens the null device (NUL on Windows) that discards all I/O.
pub fn open_null_file() -> Result<fs::File, error::Error> {
    fs::File::open("NUL").map_err(|e| error::ErrorKind::IoError(e).into())
}
