import React from 'react';
import { Button, Center, Container, Grid } from '@mantine/core';
import {
  IconShieldLock,
  IconPlayerPlay,
  IconPlayerStop,
} from '@tabler/icons-react';
import useForticlientContext from '../../context/ForticlientContext/useForticlientContext.js';
import useDocumentTitle from '../../hooks/useDocumentTitle/index.js';
import { startForticlient, stopForticlient } from '../../lib/forticlient.js';

const Home = () => {
  useDocumentTitle('forticlient-tray-helper | CodeDead');

  const { status, initialized, action, runAction } = useForticlientContext();

  const busy = !initialized || action !== null;
  const anythingRunning =
    status.service_running ||
    status.gui_running ||
    status.tray_launcher_running;
  const startDisabled = busy || anythingRunning;
  const stopDisabled = busy || !anythingRunning;

  return (
    <Container size="xl">
      <Center mt="md">
        <IconShieldLock size={80} color="var(--mantine-color-blue-filled)" />
      </Center>
      <Center mt="md">
        <Grid mt="xl">
          <Grid.Col span={6}>
            <Button
              leftSection={<IconPlayerPlay size={14} />}
              disabled={startDisabled}
              loading={action === 'Start'}
              onClick={() => runAction(startForticlient, 'Start')}
            >
              Start
            </Button>
          </Grid.Col>
          <Grid.Col span={6}>
            <Button
              leftSection={<IconPlayerStop size={14} />}
              disabled={stopDisabled}
              loading={action === 'Stop'}
              onClick={() => runAction(stopForticlient, 'Stop')}
            >
              Stop
            </Button>
          </Grid.Col>
        </Grid>
      </Center>
    </Container>
  );
};

export default Home;
