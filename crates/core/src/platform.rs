//! Platform abstraction traits for file storage and time.

use crate::error::Result;
use crate::models::{Attempt, Progress, Settings};
use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::RwLock;

pub const SETTINGS_FILE: &str = "settings.json";
pub const PROGRESS_FILE: &str = "progress.json";
pub const IN_PROGRESS_FILE: &str = "in_progress.json";

/// Abstraction for persistent file storage across desktop and mobile.
pub trait Storage: Send + Sync {
    /// Return the directory where JSON data files are stored.
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

    /// Delete file if exists.
    fn delete_file(&self, filename: &str) -> std::io::Result<()> {
        let path = self.storage_dir().join(filename);
        if path.exists() {
            std::fs::remove_file(path)?;
        }
        Ok(())
    }

    fn load_settings(&self) -> Settings {
        match self.read_file(SETTINGS_FILE) {
            Ok(content) => serde_json::from_str(&content).unwrap_or_default(),
            Err(_) => Settings::default(),
        }
    }

    fn save_settings(&self, settings: &Settings) -> Result<()> {
        let json = serde_json::to_string_pretty(settings)?;
        self.write_file(SETTINGS_FILE, &json)?;
        Ok(())
    }

    fn load_progress(&self) -> Progress {
        match self.read_file(PROGRESS_FILE) {
            Ok(content) => serde_json::from_str(&content).unwrap_or_default(),
            Err(_) => Progress::default(),
        }
    }

    fn save_progress(&self, progress: &Progress) -> Result<()> {
        let json = serde_json::to_string_pretty(progress)?;
        self.write_file(PROGRESS_FILE, &json)?;
        Ok(())
    }

    fn load_in_progress(&self) -> Option<Attempt> {
        match self.read_file(IN_PROGRESS_FILE) {
            Ok(content) => serde_json::from_str(&content).ok(),
            Err(_) => None,
        }
    }

    fn save_in_progress(&self, attempt: &Attempt) -> Result<()> {
        let json = serde_json::to_string_pretty(attempt)?;
        self.write_file(IN_PROGRESS_FILE, &json)?;
        Ok(())
    }

    fn clear_in_progress(&self) -> Result<()> {
        let _ = self.delete_file(IN_PROGRESS_FILE);
        Ok(())
    }
}

/// In-memory storage useful for unit tests and sandboxes.
#[derive(Default)]
pub struct InMemoryStorage {
    files: RwLock<HashMap<String, String>>,
}

impl InMemoryStorage {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Storage for InMemoryStorage {
    fn storage_dir(&self) -> PathBuf {
        PathBuf::from("/mock_storage")
    }

    fn read_file(&self, filename: &str) -> std::io::Result<String> {
        let map = self
            .files
            .read()
            .map_err(|_| std::io::Error::other("Lock poisoned"))?;
        map.get(filename)
            .cloned()
            .ok_or_else(|| std::io::Error::new(std::io::ErrorKind::NotFound, "File not found"))
    }

    fn write_file(&self, filename: &str, content: &str) -> std::io::Result<()> {
        let mut map = self
            .files
            .write()
            .map_err(|_| std::io::Error::other("Lock poisoned"))?;
        map.insert(filename.to_string(), content.to_string());
        Ok(())
    }

    fn delete_file(&self, filename: &str) -> std::io::Result<()> {
        let mut map = self
            .files
            .write()
            .map_err(|_| std::io::Error::other("Lock poisoned"))?;
        map.remove(filename);
        Ok(())
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

/// Mock clock for deterministic testing.
pub struct MockClock {
    current: std::sync::atomic::AtomicU64,
}

impl MockClock {
    pub fn new(initial_secs: u64) -> Self {
        Self {
            current: std::sync::atomic::AtomicU64::new(initial_secs),
        }
    }

    pub fn advance(&self, secs: u64) {
        self.current
            .fetch_add(secs, std::sync::atomic::Ordering::SeqCst);
    }

    pub fn set(&self, secs: u64) {
        self.current
            .store(secs, std::sync::atomic::Ordering::SeqCst);
    }
}

impl Clock for MockClock {
    fn now_seconds(&self) -> u64 {
        self.current.load(std::sync::atomic::Ordering::SeqCst)
    }
}
