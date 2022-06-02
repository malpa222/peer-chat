import React from 'react';
import { createRoot } from 'react-dom/client';

import Auth0Config from './auth/Auth0Config'
import App from './App';

const container = document.getElementById('root');
const root = createRoot(container);

root.render(
  <Auth0Config>
    <App />
  </Auth0Config>
);