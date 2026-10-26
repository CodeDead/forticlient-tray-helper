use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;
use std::sync::{Mutex, MutexGuard};

/// Directory name of the application config directory
const SETTINGS_DIR: &str = "forticlient-tray-helper";

/// File name of the application settings file
const SETTINGS_FILE: &str = "settings.json";

/// Guards read-modify-write cycles so concurrent setting changes cannot
/// overwrite each other.
static SETTINGS_LOCK: Mutex<()> = Mutex::new(());

/// Acquire the settings lock, recovering from poisoning so a single panicking
/// write can never permanently block future setting changes.
///
/// # Returns
///
/// A [MutexGuard] for the settings lock
fn settings_guard() -> MutexGuard<'static, ()> {
    SETTINGS_LOCK.lock().unwrap_or_else(|err| err.into_inner())
}

/// Persisted application settings. Missing fields fall back to the defaults.
#[derive(Deserialize, Serialize)]
#[serde(default)]
struct Settings {
    /// Whether FortiClient should be stopped when the application quits
    quit_forticlient_on_exit: bool,
    /// Whether FortiClient should be started when the application starts
    auto_start_forticlient: bool,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            quit_forticlient_on_exit: true,
            auto_start_forticlient: false,
        }
    }
}

/// Resolve the settings file path from the XDG config base directory,
/// falling back to `~/.config` when `XDG_CONFIG_HOME` is not set.
///
/// # Returns
///
/// A [PathBuf] pointing to the settings file
fn settings_path() -> PathBuf {
    let base = std::env::var_os("XDG_CONFIG_HOME")
        .map(PathBuf::from)
        .or_else(|| std::env::var_os("HOME").map(|home| PathBuf::from(home).join(".config")))
        .unwrap_or_else(|| PathBuf::from("."));
    base.join(SETTINGS_DIR).join(SETTINGS_FILE)
}

/// Read the persisted settings. A missing or invalid settings file falls back
/// to the defaults instead of failing.
///
/// # Returns
///
/// A [Settings] struct
fn load_settings() -> Settings {
    let Ok(contents) = fs::read_to_string(settings_path()) else {
        return Settings::default();
    };
    serde_json::from_str(&contents).unwrap_or_default()
}

/// Persist all settings, creating the settings directory when needed.
///
/// # Arguments
///
/// - `settings`: The settings to persist
///
/// # Returns
///
/// A Result containing an empty OK or an error message
fn save_settings(settings: &Settings) -> Result<(), String> {
    let path = settings_path();
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)
            .map_err(|err| format!("Failed to create settings directory: {err}"))?;
    }
    let contents = serde_json::to_string_pretty(settings)
        .map_err(|err| format!("Failed to serialize settings: {err}"))?;
    fs::write(&path, contents).map_err(|err| format!("Failed to write settings: {err}"))?;
    Ok(())
}

/// Load, modify and persist the settings in a single serialized
/// read-modify-write cycle, preserving the other settings.
///
/// # Arguments
///
/// - `update`: A callback that modifies the loaded settings
///
/// # Returns
///
/// A Result containing an empty OK or an error message
fn update_settings(update: impl FnOnce(&mut Settings)) -> Result<(), String> {
    let _guard = settings_guard();
    let mut settings = load_settings();
    update(&mut settings);
    save_settings(&settings)
}

/// Whether FortiClient should be stopped when the application quits. Any
/// problem reading the settings file falls back to the default (true).
///
/// # Returns
///
/// A boolean that indicates if FortiClient should be stopped on quit
pub(crate) fn quit_forticlient_on_exit() -> bool {
    load_settings().quit_forticlient_on_exit
}

/// Whether FortiClient should be started when the application starts. Any
/// problem reading the settings file falls back to the default (false).
///
/// # Returns
///
/// A boolean that indicates if FortiClient should be started on startup
pub(crate) fn auto_start_forticlient() -> bool {
    load_settings().auto_start_forticlient
}

/// Persist whether FortiClient should be stopped when the application quits.
///
/// # Arguments
///
/// - `value`: Whether FortiClient should be stopped on quit
///
/// # Returns
///
/// A Result containing an empty OK or an error message
pub(crate) fn set_quit_forticlient_on_exit(value: bool) -> Result<(), String> {
    update_settings(|settings| settings.quit_forticlient_on_exit = value)
}

/// Persist whether FortiClient should be started when the application starts.
///
/// # Arguments
///
/// - `value`: Whether FortiClient should be started on startup
///
/// # Returns
///
/// A Result containing an empty OK or an error message
pub(crate) fn set_auto_start_forticlient(value: bool) -> Result<(), String> {
    update_settings(|settings| settings.auto_start_forticlient = value)
}
