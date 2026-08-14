import React from 'react';
import { Card, Avatar, Text, Group, Button } from '@mantine/core';
import classes from './aboutcard.module.css';
import banner from '../../assets/about-banner.png';
import openExternal from '../../lib/openExternal.js';

const AboutCard = ({ className }) => (
  <Card withBorder radius="md" className={className}>
    <Card.Section
      h={140}
      onClick={() => openExternal('https://codedead.com/')}
      className={classes.banner}
      style={{ backgroundImage: `url(${banner})` }}
    />
    <Avatar
      src="https://codedead.com/favicon.svg"
      size={80}
      radius={80}
      mx="auto"
      mt={-30}
      onClick={() => openExternal('https://codedead.com/')}
      className={classes.avatar}
    />
    <Text
      ta="center"
      fz="lg"
      fw={500}
      mt="sm"
      className={classes.name}
      onClick={() => openExternal('https://codedead.com/')}
    >
      CodeDead
    </Text>
    <Text ta="center" fz="sm" c="dimmed">
      If you like this tool, consider donating.
    </Text>
    <Text ta="center" fz="sm" c="dimmed">
      Made with ❤️ by CodeDead.
    </Text>
    <Group mt="md" justify="center" gap={30}>
      <div>
        <Text ta="center" fz="lg" fw={500}>
          200K+
        </Text>
        <Text ta="center" fz="sm" c="dimmed" lh={1}>
          Visitors
        </Text>
      </div>
      <div>
        <Text ta="center" fz="lg" fw={500}>
          10+
        </Text>
        <Text ta="center" fz="sm" c="dimmed" lh={1}>
          Donations
        </Text>
      </div>
      <div>
        <Text ta="center" fz="lg" fw={500}>
          11
        </Text>
        <Text ta="center" fz="sm" c="dimmed" lh={1}>
          FOSS tools
        </Text>
      </div>
    </Group>
    <Button
      aria-label="Donate"
      fullWidth
      radius="md"
      mt="xl"
      size="md"
      variant="default"
      onClick={() => openExternal('https://codedead.com/donate')}
    >
      Donate
    </Button>
  </Card>
);

export default AboutCard;
