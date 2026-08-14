import { openUrl } from '@tauri-apps/plugin-opener';

/**
 * Open a URL in the system's default browser. Failures (e.g. no browser
 * available) are swallowed so the UI is never interrupted.
 */
const openExternal = (url) => {
  openUrl(url).catch(() => {});
};

export default openExternal;
