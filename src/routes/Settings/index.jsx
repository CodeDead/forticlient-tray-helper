import React from 'react';
import { Button, Checkbox, Container, Grid, Stack } from '@mantine/core';
import { notifications } from '@mantine/notifications';
import classes from './settings.module.css';
import useForticlientContext from '../../context/ForticlientContext/useForticlientContext.js';
import useDocumentTitle from '../../hooks/useDocumentTitle/index.js';
import {
  disableForticlientService,
  enableForticlientService,
  enableFortitrayLauncher,
  removeFortitrayLauncher,
  saveAutoStartFortiClient,
  saveQuitFortiClientOnExit,
} from '../../lib/forticlient.js';

const Settings = () => {
  useDocumentTitle('Settings | CodeDead');
  const { status, initialized, action, runAction, refreshStatus } =
    useForticlientContext();

  const busy = !initialized || action !== null;
  const buttonsDisabled = {
    disableService: busy || !status.service_enabled,
    enableService: busy || status.service_enabled,
    removeLauncher:
      busy || (!status.tray_launcher_running && !status.tray_autostart_enabled),
    enableLauncher:
      busy || status.tray_launcher_running || status.tray_autostart_enabled,
  };

  /**
   * Persist a setting and refresh the status
   * @param save The function that persists the setting
   * @param value The new value of the setting
   */
  const saveSetting = (save, value) => {
    save(value)
      .then(() => refreshStatus())
      .catch((err) =>
        notifications.show({
          color: 'red',
          title: 'Error',
          message: String(err),
        }),
      );
  };

  /**
   * Change quit FortiClient on exit setting
   * @param event The checked state of the checkbox
   */
  const changeQuitFortiClientOnExit = (event) => {
    saveSetting(saveQuitFortiClientOnExit, event.currentTarget.checked);
  };

  /**
   * Change auto start FortiClient setting
   * @param event The checked state of the checkbox
   */
  const changeAutoStartFortiClient = (event) => {
    saveSetting(saveAutoStartFortiClient, event.currentTarget.checked);
  };

  return (
    <Container>
      <Grid>
        <Grid.Col span={12}>
          <Stack gap="sm">
            <Checkbox
              checked={status.quit_forticlient_on_exit}
              disabled={busy}
              onChange={changeQuitFortiClientOnExit}
              label="Stop FortiClient when quitting this application"
            />
            <Checkbox
              checked={status.auto_start_forticlient}
              disabled={busy}
              onChange={changeAutoStartFortiClient}
              label="Automatically start FortiClient when this application starts"
            />
          </Stack>
        </Grid.Col>
        <Grid.Col span={12}>
          <div className={classes.buttonGroup}>
            <Button
              className={classes.button}
              disabled={buttonsDisabled.disableService}
              onClick={() => runAction(disableForticlientService, 'Disable')}
              color="red"
              loading={action === 'Disable'}
            >
              Disable forticlient service
            </Button>
            <Button
              className={classes.button}
              disabled={buttonsDisabled.enableService}
              onClick={() => runAction(enableForticlientService, 'Enable')}
              loading={action === 'Enable'}
            >
              Enable forticlient service
            </Button>
            <Button
              className={classes.button}
              disabled={buttonsDisabled.removeLauncher}
              onClick={() => runAction(removeFortitrayLauncher, 'Remove')}
              color="red"
              loading={action === 'Remove'}
            >
              Disable forticlient tray launcher
            </Button>
            <Button
              className={classes.button}
              disabled={buttonsDisabled.enableLauncher}
              onClick={() => runAction(enableFortitrayLauncher, 'Enable tray')}
              loading={action === 'Enable tray'}
            >
              Enable forticlient tray launcher
            </Button>
          </div>
        </Grid.Col>
      </Grid>
    </Container>
  );
};

export default Settings;
