//! Platform abstraction traits for file storage and time.

use std::path::PathBuf;

/// Abstraction for persistent file storage across desktop and mobile.
pub trait Storage: Send + Sync {
    /// Return the directory where JSON data files (settings.json, progress.json, in_progress.json) are stored.
    fn storage_dir(&self) -> PathBuf;

    /// Read file content as string.
    fn read_file(&self, filename: &str) -> std::io::Result<String> {
        let path = self.storage_dir().join(filename);
        std::fs::read_to_string(path)
    }

    /// Write file content atomically.
    fn write_file(&self, filename: &str, content: &str) -> std::io::Result<()> {
        let dir = self.storage_dir();
        std::fs::create_dir_all(&dir)?;
        let path = dir.join(filename);
        let tmp_path = dir.join(format!("{}.tmp", filename));
        std::fs::write(&tmp_path, content)?;
        std::fs::rename(tmp_path, path)
    }
}

/// Abstraction for clock / time to make deadline calculations testable.
pub trait Clock: Send + Sync {
    /// Return the current monotonic / wall clock time in seconds since Unix epoch.
    fn now_seconds(&self) -> u64;
}

/// Standard system clock implementation.
pub struct SystemClock;

impl Clock for SystemClock {
    fn now_seconds(&self) -> u64 {
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs()
    }
}
