import React, { ReactElement, useState, useEffect } from 'react';
import { Wasm } from '../types/Wasm';
import RenderableDropdown from './RenderableDropdown';

// async function loadWasm(
//   cb: React.Dispatch<React.SetStateAction<Wasm | undefined>>,
// ): Promise<void> {
//   const wasm = await import('wasm-app');
//   cb(wasm);
// }

interface Props {
  wasm: typeof import('wasm-app');
}

export default function WasmLoader({ wasm }: Props): ReactElement {
  return (
    <div className="container-fluid mt-5 mr-5">
      <div className="row">
        <div className="col">
          <canvas id="canvasRust" height="600" width="800" />
        </div>
        <div className="col">{wasm && <RenderableDropdown wasm={wasm} />}</div>
      </div>
    </div>
  );
}
