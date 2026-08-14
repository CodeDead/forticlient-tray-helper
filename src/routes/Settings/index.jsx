import React from 'react';
import { Button, Checkbox, Container, Grid } from '@mantine/core';
import { notifications } from '@mantine/notifications';
import classes from './settings.module.css';
import useForticlientContext from '../../context/ForticlientContext/useForticlientContext.js';
import useDocumentTitle from '../../hooks/useDocumentTitle/index.js';
import {
  disableForticlientService,
  enableForticlientService,
  enableFortitrayLauncher,
  removeFortitrayLauncher,
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
   * Change quit FortiClient on exit setting
   * @param event The checked state of the checkbox
   */
  const changeQuitFortiClientOnExit = (event) => {
    const quit = event.currentTarget.checked;
    saveQuitFortiClientOnExit(quit)
      .then(() => refreshStatus())
      .catch((err) =>
        notifications.show({
          color: 'red',
          title: 'Error',
          message: String(err),
        }),
      );
  };

  return (
    <Container>
      <Grid>
        <Grid.Col span={12}>
          <Checkbox
            checked={status.quit_forticlient_on_exit}
            disabled={busy}
            onChange={changeQuitFortiClientOnExit}
            label="Stop FortiClient when quitting this application"
          />
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
