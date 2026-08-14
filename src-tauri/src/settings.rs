use serde_json::{Value, json};
use std::fs;
use std::path::PathBuf;

/// Directory name of the application config directory
const SETTINGS_DIR: &str = "forticlient-tray-helper";

/// File name of the application settings file
const SETTINGS_FILE: &str = "settings.json";

/// Key of the "stop FortiClient when quitting" option
const KEY_QUIT_FORTICLIENT_ON_EXIT: &str = "quit_forticlient_on_exit";

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

/// Whether FortiClient should be stopped when the application quits. Any
/// problem reading the settings file falls back to the default (true).
///
/// # Returns
///
/// A boolean that indicates if FortiClient should be stopped on quit
pub(crate) fn quit_forticlient_on_exit() -> bool {
    let Ok(contents) = fs::read_to_string(settings_path()) else {
        return true;
    };
    let Ok(settings) = serde_json::from_str::<Value>(&contents) else {
        return true;
    };
    settings
        .get(KEY_QUIT_FORTICLIENT_ON_EXIT)
        .and_then(Value::as_bool)
        .unwrap_or(true)
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
    let path = settings_path();
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)
            .map_err(|err| format!("Failed to create settings directory: {err}"))?;
    }
    let contents = serde_json::to_string_pretty(&json!({ KEY_QUIT_FORTICLIENT_ON_EXIT: value }))
        .map_err(|err| format!("Failed to serialize settings: {err}"))?;
    fs::write(&path, contents).map_err(|err| format!("Failed to write settings: {err}"))?;
    Ok(())
}
