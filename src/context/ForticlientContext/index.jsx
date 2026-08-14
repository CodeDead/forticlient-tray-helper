import React from 'react';
import { ForticlientContext } from './context.js';
import useForticlient from '../../hooks/useForticlient/index.js';

/**
 * Provide a single shared FortiClient status instance to the whole app, so
 * status, busy state and alerts survive route navigation and only one
 * poller/event listener set is ever active.
 */
const ForticlientProvider = ({ children }) => {
  const value = useForticlient();
  return (
    <ForticlientContext.Provider value={value}>
      {children}
    </ForticlientContext.Provider>
  );
};

export default ForticlientProvider;
