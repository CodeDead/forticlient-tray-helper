import { useCallback, useEffect, useRef, useState } from 'react';
import { notifications } from '@mantine/notifications';
import {
  fetchStatus,
  initialStatus,
  onStatusChanged,
  onWindowFocus,
  subscribe,
} from '../../lib/forticlient.js';

/**
 * How often the status is polled while the window is visible. Status changes
 * are already pushed via "status-changed" events, so this is only a safety
 * net and does not need to be aggressive.
 */
const POLL_INTERVAL_MS = 3000;

/**
 * Shared FortiClient status handling: keeps the status in sync by polling
 * the backend while the window is visible and listening to tray/window
 * events, and provides a helper to run actions with user feedback.
 */
const useForticlient = () => {
  const [status, setStatus] = useState(initialStatus);
  const [initialized, setInitialized] = useState(false);
  const [action, setAction] = useState(null);
  const [focused, setFocused] = useState(() => document.hasFocus());
  const statusRequestId = useRef(0);
  const initializedRef = useRef(false);

  const refreshStatus = useCallback(async () => {
    const requestId = ++statusRequestId.current;
    try {
      const next = await fetchStatus();
      if (requestId !== statusRequestId.current) return;
      setStatus(next);
      setInitialized(true);
      initializedRef.current = true;
    } catch (err) {
      if (requestId !== statusRequestId.current) return;
      // Surface only an initial-load failure; transient background poll
      // failures keep the last known status instead of a new notification
      if (!initializedRef.current) {
        notifications.show({
          color: 'red',
          title: 'Error',
          message: String(err),
        });
      }
    }
  }, []);

  const runAction = useCallback(
    async (actionFn, label) => {
      setAction(label);
      try {
        await actionFn();
        notifications.show({
          color: 'green',
          title: 'Success',
          message: `${label} successful.`,
        });
        await refreshStatus();
      } catch (err) {
        notifications.show({
          color: 'red',
          title: 'Error',
          message: String(err),
        });
      } finally {
        setAction(null);
      }
    },
    [refreshStatus],
  );

  useEffect(() => {
    // eslint-disable-next-line react-hooks/set-state-in-effect
    void refreshStatus();
  }, [refreshStatus]);

  useEffect(
    () => subscribe(onStatusChanged, () => void refreshStatus()),
    [refreshStatus],
  );

  useEffect(
    () =>
      subscribe(onWindowFocus, ({ payload: isFocused }) => {
        setFocused(isFocused);
        if (isFocused) void refreshStatus();
      }),
    [refreshStatus],
  );

  useEffect(() => {
    // The window spends most of its life hidden in the tray; only poll
    // while it is visible so no systemctl processes are spawned needlessly
    if (!focused) return;
    const interval = setInterval(() => void refreshStatus(), POLL_INTERVAL_MS);
    return () => clearInterval(interval);
  }, [focused, refreshStatus]);

  return {
    status,
    initialized,
    action,
    refreshStatus,
    runAction,
  };
};

export default useForticlient;
