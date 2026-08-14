import { invoke } from '@tauri-apps/api/core';
import { listen } from '@tauri-apps/api/event';
import { getCurrentWindow } from '@tauri-apps/api/window';

export const initialStatus = {
  service_running: false,
  service_enabled: false,
  gui_running: false,
  tray_launcher_running: false,
  tray_autostart_enabled: false,
  quit_forticlient_on_exit: false,
};

export const fetchStatus = () => invoke('forticlient_status');

export const startForticlient = () => invoke('start_forticlient');

export const stopForticlient = () => invoke('stop_forticlient');

export const disableForticlientService = () =>
  invoke('disable_forticlient_service');

export const enableForticlientService = () =>
  invoke('enable_forticlient_service');

export const enableFortitrayLauncher = () =>
  invoke('enable_fortitray_launcher');

export const removeFortitrayLauncher = () =>
  invoke('remove_fortitray_launcher');

export const saveQuitFortiClientOnExit = (quit) =>
  invoke('set_quit_forticlient_on_exit', { value: quit });

export const onStatusChanged = (callback) => listen('status-changed', callback);

export const onTrayError = (callback) => listen('tray-error', callback);

export const onWindowFocus = (callback) =>
  getCurrentWindow().onFocusChanged(callback);

/**
 * StrictMode-safe subscription that also works as an effect cleanup.
 * Tauri's listen() resolves asynchronously; if cleanup runs before it
 * resolves (e.g. the StrictMode remount cycle), the resolved unlisten
 * function is invoked immediately so no listener leaks. A rejected
 * registration (e.g. IPC unavailable) is logged instead of surfacing
 * as an unhandled promise rejection.
 */
export const subscribe = (subscriber, callback) => {
  let disposed = false;
  let unlisten;
  subscriber(callback)
    .then((fn) => {
      if (disposed) fn();
      else unlisten = fn;
    })
    .catch((err) => console.warn('Failed to subscribe to event:', err));
  return () => {
    disposed = true;
    unlisten?.();
  };
};
