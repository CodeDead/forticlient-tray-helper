import { useEffect } from 'react';
import { notifications } from '@mantine/notifications';
import { onTrayError, subscribe } from '../../lib/forticlient.js';

/**
 * Surface errors pushed by the tray as a notification. Rendered once above
 * the route content so the listener survives navigation; page-initiated
 * success/error feedback is emitted directly by the useForticlient hook.
 */
const StatusAlerts = () => {
  useEffect(
    () =>
      subscribe(onTrayError, (event) =>
        notifications.show({
          color: 'red',
          title: 'Error',
          message: String(event.payload),
        }),
      ),
    [],
  );

  return null;
};

export default StatusAlerts;
