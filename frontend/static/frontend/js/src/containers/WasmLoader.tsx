import React, { ReactElement, useState, useEffect } from 'react';
import { Wasm } from '../types/Wasm';

async function loadWasm(
  cb: React.Dispatch<React.SetStateAction<Wasm | undefined>>,
): Promise<void> {
  const wasm: Wasm = await import('wasm-app');
  cb(wasm);
}

export default function WasmLoader(): ReactElement {
  const [wasm, setWasm] = useState<Wasm>();
  useEffect(() => {
    // eslint-disable-next-line @typescript-eslint/ban-types
    loadWasm(setWasm).catch((e: string) => alert(`Something went wrong when fetching wasm!\n${e}`));
  }, []);
  return <canvas id="canvasRust" height="600" width="800" />;
}
