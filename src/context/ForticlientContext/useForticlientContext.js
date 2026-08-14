import { useContext } from 'react';
import { ForticlientContext } from './context.js';

/**
 * Consume the shared FortiClient status provided by ForticlientProvider.
 */
const useForticlientContext = () => {
  const context = useContext(ForticlientContext);
  if (!context) {
    throw new Error(
      'useForticlientContext must be used within a ForticlientProvider',
    );
  }
  return context;
};

export default useForticlientContext;
