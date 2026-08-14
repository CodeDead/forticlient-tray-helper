import React from 'react';
import { Accordion, Container, Title } from '@mantine/core';
import classes from './about.module.css';
import AboutCard from '../../components/AboutCard/index.jsx';
import useDocumentTitle from '../../hooks/useDocumentTitle/index.js';

const About = () => {
  useDocumentTitle('About forticlient-tray-helper | CodeDead');

  return (
    <Container>
      <Container size="sm">
        <AboutCard className={classes.inner + ' card'} />
        <Title ta="center" className={classes.title} mt={40}>
          Frequently Asked Questions
        </Title>

        <Accordion variant="separated">
          <Accordion.Item className={classes.item} value="why">
            <Accordion.Control>Why was this tool created?</Accordion.Control>
            <Accordion.Panel>
              This tool was created to help users manage their FortiClient VPN
              connections more easily and efficiently with the ability to better
              preserve their privacy and security.
            </Accordion.Panel>
          </Accordion.Item>

          <Accordion.Item className={classes.item} value="commercial">
            <Accordion.Control>
              Can I use this tool for commercial purposes?
            </Accordion.Control>
            <Accordion.Panel>
              Yes! You can use this tool for any purpose, including commercial
              ones.
            </Accordion.Panel>
          </Accordion.Item>

          <Accordion.Item className={classes.item} value="credit-card">
            <Accordion.Control>
              Can I use this tool for free without any limitations?
            </Accordion.Control>
            <Accordion.Panel>
              Yes! You can use this tool for free without any limitations.
            </Accordion.Panel>
          </Accordion.Item>
        </Accordion>
      </Container>
    </Container>
  );
};

export default About;
