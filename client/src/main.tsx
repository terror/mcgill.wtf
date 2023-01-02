import React from 'react';

import ReactDOM from 'react-dom/client';
import App from './App';

import { ChakraProvider, extendTheme } from '@chakra-ui/react';

import '@fontsource/inter/400.css';
import '@fontsource/inter/500.css';
import '@fontsource/inter/700.css';

const theme = extendTheme({
  fonts: {
    body: `'Inter', sans-serif`,
  },
});

ReactDOM.createRoot(document.getElementById('root') as HTMLElement).render(
  <ChakraProvider theme={theme}>
    <React.StrictMode>
      <App />
    </React.StrictMode>
  </ChakraProvider>
);
