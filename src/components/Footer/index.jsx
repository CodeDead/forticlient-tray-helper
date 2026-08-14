import React from 'react';
import { Container, Group, ActionIcon, rem, Text } from '@mantine/core';
import {
  IconBrandBluesky,
  IconBrandGithub,
  IconBrandMastodon,
} from '@tabler/icons-react';
import classes from './footer.module.css';
import openExternal from '../../lib/openExternal.js';

const Footer = () => (
  <Container className={classes.inner}>
    <Text
      size="lg"
      style={{ cursor: 'pointer' }}
      onClick={() => openExternal('https://codedead.com/')}
    >
      Copyright © {new Date().getFullYear()} CodeDead
    </Text>
    <Group gap={0} className={classes.links} justify="flex-end" wrap="nowrap">
      <ActionIcon
        aria-label="BlueSky"
        size="lg"
        color="gray"
        variant="subtle"
        onClick={() => openExternal('https://bsky.app/profile/codedead.com')}
      >
        <IconBrandBluesky
          style={{ width: rem(18), height: rem(18) }}
          stroke={1.5}
        />
      </ActionIcon>
      <ActionIcon
        aria-label="Mastodon"
        size="lg"
        color="gray"
        variant="subtle"
        onClick={() => openExternal('https://mstdn.social/@CodeDead')}
      >
        <IconBrandMastodon
          style={{ width: rem(18), height: rem(18) }}
          stroke={1.5}
        />
      </ActionIcon>
      <ActionIcon
        aria-label="Github"
        size="lg"
        color="gray"
        variant="subtle"
        onClick={() => openExternal('https://github.com/CodeDead/')}
      >
        <IconBrandGithub
          style={{ width: rem(18), height: rem(18) }}
          stroke={1.5}
        />
      </ActionIcon>
    </Group>
  </Container>
);

export default Footer;
