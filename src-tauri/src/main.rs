// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

fn main() {
    #[cfg(target_os = "linux")]
    configure_linux_environment();
    forticlient_tray_helper_lib::run();
}

#[cfg(target_os = "linux")]
fn configure_linux_environment() {
    unsafe {
        std::env::set_var("__NV_DISABLE_EXPLICIT_SYNC", "1");
    }
}
