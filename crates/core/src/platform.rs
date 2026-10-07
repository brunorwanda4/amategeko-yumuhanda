//! Platform abstraction traits for file storage and time.

use crate::error::Result;
use crate::models::{Attempt, Progress, Settings};
use crate::window::{WindowState, WINDOW_BACKUP_FILE, WINDOW_FILE};
use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::{Arc, Mutex, OnceLock, RwLock};

pub const SETTINGS_FILE: &str = "settings.json";
pub const PROGRESS_FILE: &str = "progress.json";
pub const IN_PROGRESS_FILE: &str = "in_progress.json";

/// Platform service that controls whether the display may sleep.
pub trait KeepAwake: Send + Sync {
    /// Enable or disable the platform's display-sleep prevention mechanism.
    fn set(&self, on: bool);
}

struct KeepAwakeState {
    platform: Arc<dyn KeepAwake>,
    status: Mutex<KeepAwakeStatus>,
}

struct KeepAwakeStatus {
    count: usize,
    window_active: bool,
}

static KEEP_AWAKE_STATE: OnceLock<Arc<KeepAwakeState>> = OnceLock::new();

/// Register the process-wide keep-awake platform service.
pub fn register_keep_awake(platform: Arc<dyn KeepAwake>) -> bool {
    KEEP_AWAKE_STATE
        .set(Arc::new(KeepAwakeState {
            platform,
            status: Mutex::new(KeepAwakeStatus {
                count: 0,
                window_active: true,
            }),
        }))
        .is_ok()
}

/// Suspend or resume an outstanding keep-awake request with window activation.
pub fn set_keep_awake_window_active(active: bool) {
    let Some(state) = KEEP_AWAKE_STATE.get() else {
        return;
    };
    if let Ok(mut status) = state.status.lock() {
        if status.window_active != active {
            status.window_active = active;
            if status.count > 0 {
                state.platform.set(active);
            }
        }
    }
}

fn update_keep_awake_count(count: usize, acquire: bool) -> (usize, Option<bool>) {
    if acquire {
        let next = count.saturating_add(1);
        (next, (count == 0).then_some(true))
    } else if count == 0 {
        (0, None)
    } else {
        let next = count - 1;
        (next, (next == 0).then_some(false))
    }
}

/// RAII display-sleep lock shared by all active quiz renders.
pub struct KeepAwakeGuard {
    state: Arc<KeepAwakeState>,
}

impl KeepAwakeGuard {
    /// Acquire the registered keep-awake service, if the platform installed one.
    pub fn acquire() -> Option<Self> {
        let state = KEEP_AWAKE_STATE.get()?.clone();
        if let Ok(mut status) = state.status.lock() {
            let (next, transition) = update_keep_awake_count(status.count, true);
            status.count = next;
            if status.window_active {
                if let Some(on) = transition {
                    state.platform.set(on);
                }
            }
        } else {
            return None;
        }
        Some(Self { state })
    }
}

impl Drop for KeepAwakeGuard {
    fn drop(&mut self) {
        if let Ok(mut status) = self.state.status.lock() {
            let (next, transition) = update_keep_awake_count(status.count, false);
            status.count = next;
            if status.window_active {
                if let Some(on) = transition {
                    self.state.platform.set(on);
                }
            }
        }
    }
}

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

    /// Load the saved desktop window state. Returns `None` when the file is missing or
    /// corrupt; a corrupt file is first copied to `window.json.bak` so it is not lost.
    fn load_window_state(&self) -> Option<WindowState> {
        let content = self.read_file(WINDOW_FILE).ok()?;
        match serde_json::from_str(&content) {
            Ok(state) => Some(state),
            Err(_) => {
                let _ = self.write_file(WINDOW_BACKUP_FILE, &content);
                None
            }
        }
    }

    fn save_window_state(&self, state: &WindowState) -> Result<()> {
        let json = serde_json::to_string_pretty(state)?;
        self.write_file(WINDOW_FILE, &json)?;
        Ok(())
    }

    fn clear_window_state(&self) -> Result<()> {
        self.delete_file(WINDOW_FILE)?;
        Ok(())
    }

    /// Load the saved development state. Returns `None` when the file is missing or corrupt.
    #[cfg(debug_assertions)]
    fn load_dev_state(&self) -> Option<crate::dev::DevState> {
        let content = self.read_file(crate::dev::DEV_STATE_FILE).ok()?;
        match serde_json::from_str::<crate::dev::DevState>(&content) {
            Ok(state) => {
                if state.is_valid() {
                    Some(state)
                } else {
                    None
                }
            }
            Err(_) => None,
        }
    }

    #[cfg(debug_assertions)]
    fn save_dev_state(&self, state: &crate::dev::DevState) -> Result<()> {
        let json = serde_json::to_string_pretty(state)?;
        self.write_file(crate::dev::DEV_STATE_FILE, &json)?;
        Ok(())
    }

    #[cfg(debug_assertions)]
    fn clear_dev_state(&self) -> Result<()> {
        let _ = self.delete_file(crate::dev::DEV_STATE_FILE);
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

#[cfg(test)]
mod keep_awake_tests {
    use super::update_keep_awake_count;

    #[test]
    fn keep_awake_stays_on_until_last_release() {
        let (count, first) = update_keep_awake_count(0, true);
        assert_eq!((count, first), (1, Some(true)));

        let (count, second) = update_keep_awake_count(count, true);
        assert_eq!((count, second), (2, None));

        let (count, first_release) = update_keep_awake_count(count, false);
        assert_eq!((count, first_release), (1, None));

        let (count, last_release) = update_keep_awake_count(count, false);
        assert_eq!((count, last_release), (0, Some(false)));
    }
}
