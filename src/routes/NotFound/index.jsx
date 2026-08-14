import React from 'react';
import { Title, Text, Button, Container, Group } from '@mantine/core';
import { useNavigate } from 'react-router';
import classes from './notfound.module.css';
import useDocumentTitle from '../../hooks/useDocumentTitle/index.js';

const NotFound = () => {
  const navigate = useNavigate();

  useDocumentTitle('Not Found | CodeDead');

  return (
    <Container className={classes.root}>
      <div className={classes.label}>404</div>
      <Title className={classes.title}>You have found a secret place.</Title>
      <Text c="dimmed" size="lg" ta="center" className={classes.description}>
        Unfortunately, this is only a 404 page. You may have mistyped the
        address, or the page has been moved to another URL.
      </Text>
      <Group justify="center">
        <Button
          aria-label="Home"
          variant="subtle"
          size="md"
          onClick={() => navigate('/')}
        >
          Take me back to home page
        </Button>
      </Group>
    </Container>
  );
};

export default NotFound;
