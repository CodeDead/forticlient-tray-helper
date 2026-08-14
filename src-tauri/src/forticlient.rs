use serde::Serialize;
use std::process::{Command, Stdio};
use std::sync::{Mutex, MutexGuard};

#[cfg(unix)]
use std::os::unix::process::CommandExt;

/// FortiClient service name
const FORTICLIENT_SERVICE: &str = "forticlient.service";

/// FortiClient CLI; the /usr/bin/forticlient symlink created by the installer
/// points here. Prefer the absolute path so a minimal PATH (autostart,
/// desktop files) cannot break the CLI lookups.
const FORTICLIENT_CLI: &str = "/opt/forticlient/forticlient-cli";

/// Resolve the FortiClient CLI: the absolute install path first, falling back
/// to a PATH lookup for non-standard installations. None when found nowhere.
///
/// # Returns
///
/// An Option containing the CLI path to invoke
fn resolve_cli() -> Option<&'static str> {
    if std::path::Path::new(FORTICLIENT_CLI).exists() {
        return Some(FORTICLIENT_CLI);
    }
    std::env::var_os("PATH")
        .is_some_and(|path| {
            std::env::split_paths(&path).any(|dir| dir.join("forticlient").is_file())
        })
        .then_some("forticlient")
}

/// The only autostart entry this application manages. Unrelated
/// `forticlient*.desktop` entries must never be flagged or removed.
const TRAY_AUTOSTART_PATH: &str = "/etc/xdg/autostart/Fortitray.desktop";

/// FortiClient's daemons (fctsched, vpn) relaunch these binaries by absolute
/// path. Disabling the tray launcher renames them so nothing can respawn it;
/// enabling restores the original file names.
const FORTITRAY_BIN: &str = "/opt/forticlient/fortitray";
const FORTITRAY_LAUNCHER_BIN: &str = "/opt/forticlient/fortitraylauncher";
const FORTITRAY_BIN_DISABLED: &str = "/opt/forticlient/fortitray.disabled";
const FORTITRAY_LAUNCHER_BIN_DISABLED: &str = "/opt/forticlient/fortitraylauncher.disabled";
const FORTITRAY_LAUNCHER_SCRIPT: &str = "/opt/forticlient/start-fortitray-launcher.sh";
const FORTITRAY_LAUNCHER_SCRIPT_DISABLED: &str =
    "/opt/forticlient/start-fortitray-launcher.sh.disabled";

/// Guards against concurrent start/stop/enable/disable operations, which would
/// otherwise race each other (e.g. pkill running while a start is in progress).
static OP_LOCK: Mutex<()> = Mutex::new(());

/// Acquire the global operation lock, recovering from poisoning so a single
/// panicking operation can never permanently block future operations.
fn op_guard() -> MutexGuard<'static, ()> {
    // Recover from poisoning instead of bricking all future operations
    OP_LOCK.lock().unwrap_or_else(|err| err.into_inner())
}

/// Query the service's active and enabled states in a single systemctl
/// invocation. `--value` prints bare values, one per line, in the order the
/// properties were requested. When the unit does not exist, systemctl still
/// prints `inactive` and an empty UnitFileState, so both come back false.
///
/// # Returns
///
/// A Result containing an (active, enabled) tuple or an error message
pub(crate) fn service_states() -> Result<(bool, bool), String> {
    let output = Command::new("systemctl")
        .args([
            "show",
            FORTICLIENT_SERVICE,
            "--value",
            "-p",
            "ActiveState,UnitFileState",
        ])
        .stdin(Stdio::null())
        .stderr(Stdio::null())
        .output()
        .map_err(|err| format!("Failed to check forticlient service status: {err}"))?;
    if !output.status.success() {
        return Err("systemctl show forticlient.service failed".to_string());
    }
    let stdout = String::from_utf8_lossy(&output.stdout);
    let mut lines = stdout.lines();
    let active = lines.next().is_some_and(|line| line.trim() == "active");
    let enabled = lines.next().is_some_and(|line| line.trim() == "enabled");
    Ok((active, enabled))
}

/// Check whether a service is active
///
/// # Returns
///
/// A Result containing a boolean or error message
fn service_is_active() -> Result<bool, String> {
    service_states().map(|(active, _)| active)
}

/// Resolve a process's executable via /proc/PID/exe. Returns None when the
/// process is gone or a zombie: the exe symlink of a zombie cannot be read
/// (EAGAIN), and a vanished process's symlink is gone (ENOENT). Unlike
/// pgrep -f, this matches the binary a process actually runs, so an unrelated
/// command line merely mentioning a name (a shell, pgrep/pkill, the app's own
/// pkexec script) can never be reported as a running target, and concurrent
/// checks cannot match each other: no probe process is ever spawned.
///
/// # Arguments
///
/// - `pid`: The process ID to validate
///
/// # Returns
///
/// An Option containing a String
fn process_exe(pid: &str) -> Option<String> {
    std::fs::read_link(format!("/proc/{pid}/exe"))
        .ok()
        .map(|path| {
            // A process started before its binary was renamed resolves to
            // "/path (deleted)"; strip that suffix so it still matches.
            path.to_string_lossy()
                .trim_end_matches(" (deleted)")
                .to_string()
        })
}

/// Whether any currently running process matches the given predicate on its
/// resolved executable path.
fn any_process_running(matches: impl Fn(&str) -> bool) -> bool {
    let Ok(entries) = std::fs::read_dir("/proc") else {
        return false;
    };
    for entry in entries.flatten() {
        let file_name = entry.file_name();
        let Some(name) = file_name.to_str() else {
            continue;
        };
        if !name.bytes().all(|b| b.is_ascii_digit()) {
            continue;
        }
        if process_exe(name).is_some_and(|exe| matches(&exe)) {
            return true;
        }
    }
    false
}

/// Check whether the FortiClient GUI is currently running.
///
/// # Returns
///
/// A boolean that indicates if the FortiClient GUI is running or not
pub(crate) fn gui_is_running() -> bool {
    any_process_running(|exe| exe.starts_with("/opt/forticlient/gui/"))
}

/// Check whether the FortiClient tray launcher binary is currently running.
///
/// # Returns
///
/// A boolean that indicates if the FortiClient Tray application is running or not
pub(crate) fn tray_launcher_active() -> bool {
    any_process_running(|exe| exe == FORTITRAY_BIN || exe == FORTITRAY_LAUNCHER_BIN)
}

/// Checks whether the tray launcher autostart entry currently exists.
///
/// # Returns
///
/// A boolean that indicates if the tray icon is automatically started
pub(crate) fn tray_autostart_enabled() -> bool {
    // symlink_metadata does not follow the final link, so a dangling
    // symlink still counts as a present (and removable) entry
    std::fs::symlink_metadata(TRAY_AUTOSTART_PATH).is_ok()
}

/// Run pkill with the given arguments, treating both "processes matched"
/// (exit code 0) and "no processes matched" (exit code 1) as a success.
///
/// # Arguments
///
/// - `args`: pkill arguments to execute
///
/// # Returns
///
/// A Result containing an empty OK or an error message
fn pkill(args: &[&str]) -> Result<(), String> {
    let status = Command::new("pkill")
        .args(args)
        .status()
        .map_err(|err| format!("Failed to run pkill: {err}"))?;

    match status.code() {
        Some(0) | Some(1) => Ok(()),
        Some(code) => Err(format!("pkill failed with exit code {code}")),
        None => Err("pkill was terminated by a signal".to_string()),
    }
}

/// A snapshot of the FortiClient service, GUI and tray launcher state,
/// returned to the frontend.
#[derive(Serialize)]
pub(crate) struct ForticlientStatus {
    service_running: bool,
    service_enabled: bool,
    gui_running: bool,
    tray_launcher_running: bool,
    tray_autostart_enabled: bool,
    quit_forticlient_on_exit: bool,
}

/// Collect the current status of the FortiClient service, GUI and tray
/// launcher.
///
/// # Returns
///
/// A Result containing a [ForticlientStatus] or an error message
pub(crate) fn build_status() -> Result<ForticlientStatus, String> {
    let (active, enabled) = service_states()?;
    Ok(ForticlientStatus {
        service_running: active,
        service_enabled: enabled,
        gui_running: gui_is_running(),
        tray_launcher_running: tray_launcher_active(),
        tray_autostart_enabled: tray_autostart_enabled(),
        quit_forticlient_on_exit: crate::settings::quit_forticlient_on_exit(),
    })
}

/// Whether FortiClient is considered running, i.e. the service is active or
/// the GUI is running.
///
/// # Returns
///
/// True if the service is running, otherwise false
pub(crate) fn is_running() -> bool {
    service_is_active().unwrap_or(false) || gui_is_running()
}

/// Launch a program fully detached: no stdio, its own session (setsid) so it
/// survives the app, and no wait on it.
///
/// # Arguments
///
/// - `program`: The program to spawn detached
/// - `args`: The application arguments to launch the program with
///
/// # Returns
///
/// A Result containing an empty OK or an error message
fn spawn_detached(program: &str, args: &[&str]) -> Result<(), String> {
    let mut command = Command::new(program);
    command
        .args(args)
        .stdin(Stdio::null())
        .stdout(Stdio::null())
        .stderr(Stdio::null());

    #[cfg(unix)]
    // SAFETY: The child process has no threads running at this point and does not
    // share a process group ID with the parent, so setsid() cannot fail here.
    unsafe {
        command.pre_exec(|| {
            libc::setsid();
            Ok(())
        });
    }

    command
        .spawn()
        .map(|mut child| {
            std::thread::spawn(move || {
                let _ = child.wait();
            });
        })
        .map_err(|err| format!("Failed to launch {program}: {err}"))?;
    Ok(())
}

/// Whether the forticlient daemon responds to a version query, i.e. whether
/// it has fully loaded.
///
/// # Arguments
///
/// - `cli`: The resolved FortiClient CLI path
///
/// # Returns
///
/// True if the daemon is ready, otherwise false
fn daemon_is_ready(cli: &str) -> bool {
    Command::new(cli)
        .args(["version"])
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status()
        .map(|status| status.success())
        .unwrap_or(false)
}

/// Poll daemon readiness until it succeeds or the given timeout elapses.
///
/// # Arguments
///
/// - `cli`: The resolved FortiClient CLI path
/// - `timeout`: The maximum [std::time::Duration] to wait for
///
/// # Returns
///
/// A Result containing an empty OK or an error message
fn wait_until_ready(cli: &str, timeout: std::time::Duration) -> Result<(), String> {
    let deadline = std::time::Instant::now() + timeout;
    while std::time::Instant::now() < deadline {
        if daemon_is_ready(cli) {
            return Ok(());
        }
        std::thread::sleep(std::time::Duration::from_millis(500));
    }
    Err("forticlient daemon did not become ready within the timeout".to_string())
}

/// Run a command as root through pkexec and return a user-readable error,
/// mapping cancelled (126) and denied (127) authentication to hints.
///
/// # Arguments
///
/// - `args`: The pkexec arguments
///
/// # Returns
///
/// A Result containing an empty OK or an error message
fn run_pkexec(args: &[&str]) -> Result<(), String> {
    let output = Command::new("pkexec")
        .args(args)
        .output()
        .map_err(|err| format!("Failed to launch pkexec: {err}"))?;

    if !output.status.success() {
        let code = output
            .status
            .code()
            .map(|c| c.to_string())
            .unwrap_or_else(|| "unknown".to_string());
        let stderr = String::from_utf8_lossy(&output.stderr).trim().to_string();
        let message = if stderr.is_empty() {
            format!("Command failed with exit code {code}")
        } else {
            format!("Command failed with exit code {code}: {stderr}")
        };
        return Err(match code.as_str() {
            "126" => format!("{message}. Authentication was cancelled or dismissed."),
            "127" => format!("{message}. You are not authorized to run this command."),
            _ => message,
        });
    }

    Ok(())
}

/// Start the forticlient service (unless already active) and show the GUI,
/// waiting for the daemon to be fully loaded first. Serialized via the
/// global operation lock.
///
/// # Returns
///
/// A Result containing an empty OK or an error message
pub(crate) fn start_forticlient_sync() -> Result<(), String> {
    let _guard = op_guard();
    // Resolve the CLI before prompting for root or waiting on the daemon so
    // a missing installation fails fast with a clear error
    let cli = resolve_cli().ok_or_else(|| {
        format!("FortiClient CLI not found at {FORTICLIENT_CLI} or on PATH")
    })?;
    if !service_is_active()? {
        run_pkexec(&["systemctl", "start", FORTICLIENT_SERVICE])?;
    }

    // Wait until the daemon is fully loaded before launching the GUI
    wait_until_ready(cli, std::time::Duration::from_secs(30))?;

    spawn_detached(cli, &["gui", "show"])
}

/// Stop the forticlient service (if active) and kill the GUI and tray
/// launcher processes. Serialized via the global operation lock.
///
/// # Returns
///
/// A Result containing an empty OK or an error message
pub(crate) fn stop_forticlient_sync() -> Result<(), String> {
    let _guard = op_guard();
    pkill(&["-f", "/opt/forticlient/gui/"])?;
    pkill(&["-f", "fortitraylauncher"])?;
    pkill(&["-x", "fortitray"])?;
    pkill(&["-x", "FortiClient"])?;
    if service_is_active()? {
        run_pkexec(&["systemctl", "stop", FORTICLIENT_SERVICE])?;
    }
    Ok(())
}

/// Stop and disable the forticlient service, prompting for root once.
/// Serialized via the global operation lock.
///
/// # Returns
///
/// A Result containing an empty OK or an error message
pub(crate) fn disable_forticlient_service_sync() -> Result<(), String> {
    let _guard = op_guard();
    // Stop and disable in a single pkexec invocation so the user only
    // gets one polkit prompt
    let script = format!(
        "set -e\nsystemctl disable {FORTICLIENT_SERVICE}"
    );
    run_pkexec(&["/bin/sh", "-c", script.as_str()])
}

/// Enable and start the forticlient service, prompting for root once.
/// Serialized via the global operation lock.
///
/// # Returns
///
/// A Result containing an empty OK or an error message
pub(crate) fn enable_forticlient_service_sync() -> Result<(), String> {
    let _guard = op_guard();
    // Enable and start in a single pkexec invocation so the user only
    // gets one polkit prompt
    let script = format!(
        "set -e\nsystemctl enable {FORTICLIENT_SERVICE}"
    );
    run_pkexec(&["/bin/sh", "-c", script.as_str()])
}

/// Restore the tray launcher binaries and autostart entry in a single
/// pkexec invocation, then launch the launcher.
///
/// # Returns
///
/// A Result containing an empty OK or an error message
pub(crate) fn enable_fortitray_launcher_sync() -> Result<(), String> {
    let _guard = op_guard();
    // Restore the binaries and autostart entry in a single pkexec invocation
    // so the user only gets one polkit prompt
    let script = format!(
        "set -e\n\
         if [ -e {FORTITRAY_BIN_DISABLED} ]; then \
         mv -f {FORTITRAY_BIN_DISABLED} {FORTITRAY_BIN}; fi\n\
         if [ -e {FORTITRAY_LAUNCHER_BIN_DISABLED} ]; then \
         mv -f {FORTITRAY_LAUNCHER_BIN_DISABLED} {FORTITRAY_LAUNCHER_BIN}; fi\n\
         if [ -e {FORTITRAY_LAUNCHER_SCRIPT_DISABLED} ]; then \
         mv -f {FORTITRAY_LAUNCHER_SCRIPT_DISABLED} {FORTITRAY_LAUNCHER_SCRIPT}; fi\n\
         if [ ! -e {TRAY_AUTOSTART_PATH} ]; then \
         ln -sf /opt/forticlient/Fortitray.desktop {TRAY_AUTOSTART_PATH}; fi"
    );
    run_pkexec(&["/bin/sh", "-c", script.as_str()])?;

    // Only launch when no instance is running yet, so enabling the launcher
    // can never result in duplicate tray processes
    if !tray_launcher_active() {
        spawn_detached(FORTITRAY_LAUNCHER_BIN, &[])?;
    }
    Ok(())
}

/// Disable the tray launcher: rename its binaries so FortiClient's daemons
/// can no longer respawn it, remove the autostart entry, and kill any
/// running instance. All privileged steps happen in a single pkexec
/// invocation.
///
/// # Returns
///
/// A Result containing an empty OK or an error message
pub(crate) fn remove_fortitray_launcher_sync() -> Result<(), String> {
    let _guard = op_guard();
    // Rename the binaries first so FortiClient's daemons can no longer respawn
    // the tray launcher, then remove the autostart entry. All of it happens in
    // a single pkexec invocation so the user only gets one polkit prompt.
    let script = format!(
        "set -e\n\
         if [ -e {FORTITRAY_BIN} ]; then \
         mv -f {FORTITRAY_BIN} {FORTITRAY_BIN_DISABLED}; fi\n\
         if [ -e {FORTITRAY_LAUNCHER_BIN} ]; then \
         mv -f {FORTITRAY_LAUNCHER_BIN} {FORTITRAY_LAUNCHER_BIN_DISABLED}; fi\n\
         if [ -e {FORTITRAY_LAUNCHER_SCRIPT} ]; then \
         mv -f {FORTITRAY_LAUNCHER_SCRIPT} {FORTITRAY_LAUNCHER_SCRIPT_DISABLED}; fi\n\
         rm -f {TRAY_AUTOSTART_PATH}"
    );
    run_pkexec(&["/bin/sh", "-c", script.as_str()])?;

    pkill(&["-f", "fortitraylauncher"])?;
    pkill(&["-x", "fortitray"])?;
    Ok(())
}
