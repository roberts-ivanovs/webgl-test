import React, { ReactElement, useState, useEffect } from 'react';
import RenderableDropdown from './RenderableDropdown';
import { CanvasData } from "wasm-app";

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
  const [canvas, setCanvas] = useState<CanvasData>();
  const [width, setWidth] = useState(800);
  const [degrees, setDegrees] = useState(45);
  const [height, setHeight] = useState(800);
  const canvasId = "canvasRust";

  useEffect(() => {
    setCanvas(new wasm.CanvasData(width, height, degrees, canvasId))
  }, [width, height, canvasId]);

  return (
    <div className="container-fluid mt-5 mr-5">
      <div className="row">
        <div className="col">
          <canvas id={canvasId} height={height} width={width} />
        </div>
        <div className="col">
          {wasm && canvas && <RenderableDropdown wasm={wasm} canvas={canvas} />}
        </div>
      </div>
    </div>
  );
}
