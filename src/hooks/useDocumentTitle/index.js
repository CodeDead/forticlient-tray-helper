import { useEffect } from 'react';

/**
 * Set the document title
 * @param title The title to apply to the document
 */
const useDocumentTitle = (title) => {
  useEffect(() => {
    document.title = title;
  }, [title]);
};

export default useDocumentTitle;
