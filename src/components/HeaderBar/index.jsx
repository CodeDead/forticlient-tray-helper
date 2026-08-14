import React from 'react';
import {
  Container,
  Group,
  Burger,
  Title,
  useMantineColorScheme,
  Tooltip,
  Drawer,
  ScrollArea,
  Divider,
  rem,
  ActionIcon,
} from '@mantine/core';
import { useDisclosure } from '@mantine/hooks';
import { IconSun, IconSunOff } from '@tabler/icons-react';
import { Link, useLocation } from 'react-router';
import classes from './headerbar.module.css';

const LINKS = [
  { to: '/', label: 'Home' },
  { to: '/settings', label: 'Settings' },
  { to: '/about', label: 'About' },
];

const NavLinks = ({ onNavigate }) => {
  const { pathname } = useLocation();
  return LINKS.map(({ to, label }) => (
    <Link
      key={to}
      to={to}
      className={classes.link}
      data-active={pathname === to ? true : undefined}
      onClick={onNavigate}
    >
      {label}
    </Link>
  ));
};

const HeaderBar = () => {
  const { toggleColorScheme } = useMantineColorScheme();
  const [opened, { toggle }] = useDisclosure(false);

  return (
    <Container size="xl" className={classes.inner}>
      <Title order={2}>forticlient-tray-helper</Title>
      <Group gap={5} visibleFrom="xs">
        <NavLinks />
      </Group>

      <Tooltip label="Light">
        <ActionIcon
          aria-label="Theme"
          variant="subtle"
          onClick={toggleColorScheme}
          lightHidden
          visibleFrom="xs"
        >
          <IconSun style={{ width: '70%', height: '70%' }} stroke={1.5} />
        </ActionIcon>
      </Tooltip>

      <Tooltip label="Dark">
        <ActionIcon
          aria-label="Theme"
          variant="subtle"
          onClick={toggleColorScheme}
          darkHidden
          visibleFrom="xs"
        >
          <IconSunOff style={{ width: '70%', height: '70%' }} stroke={1.5} />
        </ActionIcon>
      </Tooltip>

      <Burger
        aria-label="Burger menu"
        opened={opened}
        onClick={toggle}
        hiddenFrom="xs"
        size="sm"
      />
      <Drawer
        opened={opened}
        onClose={() => toggle()}
        size="100%"
        padding="md"
        title="Navigation"
        hiddenFrom="sm"
        zIndex={1000000}
      >
        <ScrollArea h={`calc(100vh - ${rem(80)})`} mx="-md">
          <Divider my="sm" />
          <NavLinks onNavigate={toggle} />
        </ScrollArea>
      </Drawer>
    </Container>
  );
};

export default HeaderBar;
