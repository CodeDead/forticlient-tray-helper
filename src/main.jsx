import { StrictMode } from 'react';
import { MantineProvider } from '@mantine/core';
import { Notifications } from '@mantine/notifications';
import { createRoot } from 'react-dom/client';
import '@mantine/core/styles.css';
import '@mantine/notifications/styles.css';
import App from './components/App/index.jsx';

createRoot(document.getElementById('root')).render(
  <StrictMode>
    <MantineProvider defaultColorScheme="auto">
      <Notifications position="bottom-right" />
      <App />
    </MantineProvider>
  </StrictMode>,
);
