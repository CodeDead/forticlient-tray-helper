mod commands;
mod forticlient;
mod settings;
mod tray;

use tauri::{Emitter, tray::TrayIconBuilder};

use commands::{
    disable_forticlient_service, enable_forticlient_service, enable_fortitray_launcher,
    forticlient_status, remove_fortitray_launcher, set_auto_start_forticlient,
    set_quit_forticlient_on_exit, start_forticlient, stop_forticlient,
};
use tray::{
    TRAY_ID, handle_tray_icon_event, handle_tray_menu_event, refresh_tray_menu, show_window,
    tray_icon,
};

/// Application entry point: sets up the tray icon and menu, registers the
/// Tauri commands and starts the runtime.
#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_single_instance::init(|app, _args, _cwd| {
            show_window(app);
        }))
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            forticlient_status,
            start_forticlient,
            stop_forticlient,
            disable_forticlient_service,
            enable_forticlient_service,
            enable_fortitray_launcher,
            remove_fortitray_launcher,
            set_quit_forticlient_on_exit,
            set_auto_start_forticlient,
        ])
        .on_window_event(|window, event| {
            if let tauri::WindowEvent::CloseRequested { api, .. } = event {
                let _ = window.hide();
                api.prevent_close();
            }
        })
        .setup(|app| {
            TrayIconBuilder::with_id(TRAY_ID)
                .icon(tray_icon(false).expect("missing tray icon").clone())
                .tooltip("forticlient-tray-helper")
                .on_menu_event(handle_tray_menu_event)
                .on_tray_icon_event(handle_tray_icon_event)
                .build(app)?;
            refresh_tray_menu(app.handle())?;

            // Start FortiClient in the background when the user enabled it.
            // Running this on the main thread would delay the tray and UI
            // while waiting for a polkit prompt or the daemon.
            if settings::auto_start_forticlient() {
                let app_handle = app.handle().clone();
                std::thread::spawn(move || {
                    if let Err(err) = forticlient::start_forticlient_sync() {
                        eprintln!("Failed to start FortiClient automatically: {err}");
                        let _ = app_handle.emit(
                            "tray-error",
                            format!("Failed to start FortiClient automatically: {err}"),
                        );
                    }
                    let _ = refresh_tray_menu(&app_handle);
                });
            }

            // Fallback refresh for changes made outside the app (e.g. a
            // manual systemctl call). App-driven changes refresh the tray
            // immediately, so this only needs to run at a low frequency.
            let app_handle = app.handle().clone();
            std::thread::spawn(move || {
                loop {
                    std::thread::sleep(std::time::Duration::from_secs(5));
                    let _ = refresh_tray_menu(&app_handle);
                }
            });
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
