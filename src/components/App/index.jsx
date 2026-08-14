import React, { Suspense, lazy } from 'react';
import { AppShell, Center, Loader } from '@mantine/core';
import { HashRouter, Routes, Route } from 'react-router';
import ForticlientProvider from '../../context/ForticlientContext/index.jsx';
import Footer from '../Footer/index.jsx';
import HeaderBar from '../HeaderBar';
import StatusAlerts from '../StatusAlerts/index.jsx';

const Home = lazy(() => import('../../routes/Home'));
const About = lazy(() => import('../../routes/About'));
const Settings = lazy(() => import('../../routes/Settings'));
const NotFound = lazy(() => import('../../routes/NotFound'));

const App = () => {
  return (
    <HashRouter>
      <ForticlientProvider>
        <AppShell header={{ height: 60 }} footer={{ height: 65 }} padding="md">
          <AppShell.Header>
            <HeaderBar />
          </AppShell.Header>
          <AppShell.Main>
            <StatusAlerts />
            <Suspense
              fallback={
                <Center h={100}>
                  <Loader type="bars" />
                </Center>
              }
            >
              <Routes>
                <Route path="/" element={<Home />} />
                <Route path="/settings" element={<Settings />} />
                <Route path="/about" element={<About />} />
                <Route path="*" element={<NotFound />} />
              </Routes>
            </Suspense>
          </AppShell.Main>
          <AppShell.Footer visibleFrom="xs">
            <Footer />
          </AppShell.Footer>
        </AppShell>
      </ForticlientProvider>
    </HashRouter>
  );
};

export default App;
