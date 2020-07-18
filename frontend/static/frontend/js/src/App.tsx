import React from 'react';
import ReactDOM from 'react-dom';

import 'bootstrap/dist/css/bootstrap.min.css';
import 'bootstrap/dist/js/bootstrap.min';
import WasmLoader from './containers/WasmLoader';

const rust = import('wasm-app');

rust.then((m) => {
  ReactDOM.render(<WasmLoader wasm={m} />, document.getElementById('root'));
}).catch((e) => {
  // eslint-disable-next-line @typescript-eslint/restrict-template-expressions
  console.log(`Something went wrong when fetching wasm!\n${e}`);
});
