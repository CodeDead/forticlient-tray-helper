import { createContext } from 'react';

/**
 * Shared FortiClient status context. Kept in its own file so the provider
 * component and the consumer hook can both import it without tripping
 * react-refresh/only-export-components.
 */
export const ForticlientContext = createContext(null);
