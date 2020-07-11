import React, { ReactElement, useState, useEffect } from 'react';
import { Wasm } from '../types/Wasm';

async function loadWasm(
  cb: React.Dispatch<React.SetStateAction<Wasm | undefined>>,
): Promise<void> {
  const wasm = await import('wasm-app');
  cb(wasm);
}

export default function WasmLoader(): ReactElement {
  const [wasm, setWasm] = useState<Wasm>();
  useEffect(() => {
    loadWasm(setWasm).catch((e) => console.log('Something went wrong when fetching wasm'));
  }, []);
  wasm?.greet();
  return <div>Hello</div>;
}
