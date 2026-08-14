use std::sync::{Mutex, OnceLock};

use tauri::{
    AppHandle, Emitter, Manager,
    image::Image,
    menu::{Menu, MenuItem, PredefinedMenuItem},
    tray::{MouseButton, MouseButtonState, TrayIconEvent},
};

use crate::forticlient::{
    gui_is_running, is_running, service_states, start_forticlient_sync, stop_forticlient_sync,
    tray_autostart_enabled, tray_launcher_active,
};

pub(crate) const TRAY_ID: &str = "main";
const TRAY_ITEM_TOGGLE: &str = "toggle";
const TRAY_ITEM_SHOW: &str = "show";
const TRAY_ITEM_QUIT: &str = "quit";

static LAST_STATE: Mutex<Option<TrayState>> = Mutex::new(None);

static TRAY_RUNNING_ICON: OnceLock<Image<'static>> = OnceLock::new();
static TRAY_STOPPED_ICON: OnceLock<Image<'static>> = OnceLock::new();

/// Load the tray icon for the given FortiClient running state.
pub(crate) fn tray_icon(running: bool) -> Option<&'static Image<'static>> {
    let icon = if running {
        &TRAY_RUNNING_ICON
    } else {
        &TRAY_STOPPED_ICON
    };
    icon.get_or_init(|| {
        let bytes: &[u8] = if running {
            include_bytes!("../icons/tray-running.png")
        } else {
            include_bytes!("../icons/tray-stopped.png")
        };
        Image::from_bytes(bytes).expect("invalid embedded tray icon")
    });
    icon.get()
}

/// A snapshot of the state the tray menu depends on, used to decide whether
/// the menu needs rebuilding.
#[derive(PartialEq)]
struct TrayState {
    running: bool,
    service_enabled: bool,
    tray_launcher_running: bool,
    tray_autostart_enabled: bool,
}

/// Collect the state the tray menu depends on.
///
/// # Returns
///
/// A [TrayState] struct
fn build_tray_state() -> TrayState {
    // Single systemctl invocation for both flags; treat query failures as
    // "not running/not enabled" so the tray keeps working without systemd
    let (active, enabled) = service_states().unwrap_or((false, false));
    TrayState {
        running: active || gui_is_running(),
        service_enabled: enabled,
        tray_launcher_running: tray_launcher_active(),
        tray_autostart_enabled: tray_autostart_enabled(),
    }
}

/// Show, unminimize, and focus the main window.
pub(crate) fn show_window(app: &AppHandle) {
    if let Some(window) = app.get_webview_window("main") {
        // On Linux/GNOME unminimize only works reliably on a hidden window:
        // https://github.com/tauri-apps/tauri/issues/5974
        if window.is_minimized().unwrap_or(false) {
            let _ = window.hide();
        }
        let _ = window.unminimize();
        let _ = window.show();
        let _ = window.set_focus();
        // Workaround for https://github.com/tauri-apps/tauri/issues/11856:
        // on Linux/Wayland the titlebar buttons stop responding after a
        // hide/show cycle; toggling the resizable property restores them.
        // Remove once tauri >= 2.12.0 ships the fix (tao PR #1218).
        #[cfg(target_os = "linux")]
        {
            let _ = window.set_resizable(false);
            let _ = window.set_resizable(true);
        }
    }
}

/// Rebuild the tray menu when the underlying state has changed and notify
/// the frontend of the running state via the "status-changed" event.
pub(crate) fn refresh_tray_menu(app: &AppHandle) -> tauri::Result<()> {
    let state = build_tray_state();
    let running = state.running;
    {
        let mut last = LAST_STATE.lock().unwrap_or_else(|err| err.into_inner());
        if last.as_ref() == Some(&state) {
            return Ok(());
        }
        *last = Some(state);
    }

    let toggle = MenuItem::with_id(
        app,
        TRAY_ITEM_TOGGLE,
        if running {
            "Stop FortiClient"
        } else {
            "Start FortiClient"
        },
        true,
        None::<&str>,
    )?;
    let show = MenuItem::with_id(app, TRAY_ITEM_SHOW, "Show window", true, None::<&str>)?;
    let separator = PredefinedMenuItem::separator(app)?;
    let quit = MenuItem::with_id(app, TRAY_ITEM_QUIT, "Quit", true, None::<&str>)?;

    let menu = Menu::with_items(app, &[&toggle, &show, &separator, &quit])?;
    if let Some(tray) = app.tray_by_id(TRAY_ID) {
        tray.set_menu(Some(menu))?;
        tray.set_icon(tray_icon(running).cloned())?;
    }

    // Let the frontend know the running state changed so open windows can refresh
    let _ = app.emit("status-changed", running);
    Ok(())
}

/// Handle tray menu clicks: toggle start/stop FortiClient, show the window,
/// or quit (stopping FortiClient first if enabled in the settings).
pub(crate) fn handle_tray_menu_event(handle: &AppHandle, event: tauri::menu::MenuEvent) {
    match event.id().as_ref() {
        TRAY_ITEM_TOGGLE => {
            let app_handle = handle.clone();
            tauri::async_runtime::spawn_blocking(move || {
                let result = if is_running() {
                    stop_forticlient_sync()
                } else {
                    start_forticlient_sync()
                };
                if let Err(err) = result {
                    let _ = app_handle.emit("tray-error", err);
                }
                let _ = refresh_tray_menu(&app_handle);
            });
        }
        TRAY_ITEM_SHOW => show_window(handle),
        TRAY_ITEM_QUIT => {
            let app_handle = handle.clone();
            tauri::async_runtime::spawn_blocking(move || {
                // Only stop FortiClient when the user enabled it in the
                // settings; stopping it prompts for root via pkexec
                if is_running() && crate::settings::quit_forticlient_on_exit() {
                    let _ = stop_forticlient_sync();
                }
                app_handle.exit(0);
            });
        }
        _ => {}
    }
}

/// Show the main window when the tray icon is left-clicked.
pub(crate) fn handle_tray_icon_event(tray: &tauri::tray::TrayIcon, event: TrayIconEvent) {
    if let TrayIconEvent::Click {
        button: MouseButton::Left,
        button_state: MouseButtonState::Up,
        ..
    } = event
    {
        show_window(tray.app_handle());
    }
}
