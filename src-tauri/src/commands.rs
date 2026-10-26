use crate::forticlient::{
    ForticlientStatus, build_status, disable_forticlient_service_sync,
    enable_forticlient_service_sync, enable_fortitray_launcher_sync,
    remove_fortitray_launcher_sync, start_forticlient_sync, stop_forticlient_sync,
};
use crate::settings;
use crate::tray::refresh_tray_menu;
use tauri::AppHandle;

/// Run blocking work on the async runtime's blocking thread pool so the main
/// thread (and therefore the UI) stays responsive.
async fn run_blocking<T: Send + 'static>(
    f: impl FnOnce() -> Result<T, String> + Send + 'static,
) -> Result<T, String> {
    tauri::async_runtime::spawn_blocking(f)
        .await
        .map_err(|err| format!("Background task failed: {err}"))?
}

/// Run an operation on the blocking pool, then refresh the tray menu. The
/// refresh happens even on failure: a partial operation may still have
/// changed state, and refresh_tray_menu is a no-op when nothing changed.
async fn run_and_refresh(
    app: &AppHandle,
    op: impl FnOnce() -> Result<(), String> + Send + 'static,
) -> Result<(), String> {
    let result = run_blocking(op).await;
    let _ = refresh_tray_menu(app);
    result
}

/// Fetch the current FortiClient status for the frontend.
///
/// # Returns
///
/// A Result containing a [ForticlientStatus] or an error message
#[tauri::command]
pub async fn forticlient_status() -> Result<ForticlientStatus, String> {
    run_blocking(build_status).await
}

/// Start FortiClient (service and GUI) for the frontend.
///
/// # Returns
///
/// A Result containing an empty OK or an error message
#[tauri::command]
pub async fn start_forticlient(app: AppHandle) -> Result<(), String> {
    run_and_refresh(&app, start_forticlient_sync).await
}

/// Stop FortiClient (service and GUI) for the frontend.
///
/// # Returns
///
/// A Result containing an empty OK or an error message
#[tauri::command]
pub async fn stop_forticlient(app: AppHandle) -> Result<(), String> {
    run_and_refresh(&app, stop_forticlient_sync).await
}

/// Stop and disable the forticlient service, prompting for root once.
///
/// # Returns
///
/// A Result containing an empty OK or an error message
#[tauri::command]
pub async fn disable_forticlient_service(app: AppHandle) -> Result<(), String> {
    run_and_refresh(&app, disable_forticlient_service_sync).await
}

/// Enable and start the forticlient service, prompting for root once.
///
/// # Returns
///
/// A Result containing an empty OK or an error message
#[tauri::command]
pub async fn enable_forticlient_service(app: AppHandle) -> Result<(), String> {
    run_and_refresh(&app, enable_forticlient_service_sync).await
}

/// Re-enable and launch the FortiClient tray launcher for the frontend.
///
/// # Returns
///
/// A Result containing an empty OK or an error message
#[tauri::command]
pub async fn enable_fortitray_launcher(app: AppHandle) -> Result<(), String> {
    run_and_refresh(&app, enable_fortitray_launcher_sync).await
}

/// Disable the FortiClient tray launcher for the frontend.
///
/// # Returns
///
/// A Result containing an empty OK or an error message
#[tauri::command]
pub async fn remove_fortitray_launcher(app: AppHandle) -> Result<(), String> {
    run_and_refresh(&app, remove_fortitray_launcher_sync).await
}

/// Set whether FortiClient should be stopped when the application quits.
///
/// # Arguments
///
/// - `value` - Whether FortiClient should be stopped on quit
///
/// # Returns
///
/// A Result containing an empty OK or an error message
#[tauri::command]
pub async fn set_quit_forticlient_on_exit(value: bool) -> Result<(), String> {
    run_blocking(move || settings::set_quit_forticlient_on_exit(value)).await
}

/// Set whether FortiClient should be started when the application starts.
///
/// # Arguments
///
/// - `value` - Whether FortiClient should be started on startup
///
/// # Returns
///
/// A Result containing an empty OK or an error message
#[tauri::command]
pub async fn set_auto_start_forticlient(value: bool) -> Result<(), String> {
    run_blocking(move || settings::set_auto_start_forticlient(value)).await
}
