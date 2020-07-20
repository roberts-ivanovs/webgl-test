import React, { ReactElement, useState, useEffect, useCallback } from 'react';
import { GlClient, RenderableOption, CanvasData, Transform } from 'wasm-app';

interface Props {
  wasm: typeof import('wasm-app');
  canvas: CanvasData;
}

const FPS_THROTTLE = 1000 / 60; // 60fps


interface Current {
  current: number
}

export default function RenderableDropdown({
  wasm,
  canvas,
}: Props): ReactElement {
  const [options, setOption] = useState<RenderableOption>();
  const [client, setClient] = useState<GlClient>();
  const [x, setX] = useState(0);
  const [y, setY] = useState(0);
  const [z, setZ] = useState(-6);

  // Use useRef for mutable variables that we want to persist
  // without triggering a re-render on their change
  const requestRef = React.useRef<number>();
  const previousTimeRef = React.useRef<number>();

  const animate = useCallback(
    (time: number) => {
      if (
        previousTimeRef.current != undefined &&
        time - previousTimeRef.current > FPS_THROTTLE
      ) {
        client?.render();
      }
      previousTimeRef.current = time;
      requestRef.current = requestAnimationFrame(animate);
    },
    [client === undefined],
  );

  useEffect(() => {
    requestRef.current = requestAnimationFrame(animate);
    return () => cancelAnimationFrame(requestRef.current!);
  }, [client === undefined]); // Make sure the effect runs only once

  useEffect(() => {
    if (options === undefined) {
      setOption(+wasm.RenderableOption.Cube);
    } else {
      const defTransform = new wasm.Transform(0, 0, -6);
      if (client === undefined) {
        const tmpClient = new wasm.GlClient(+options, canvas, defTransform);
        setClient(tmpClient);
      } else {
        client.set_renderable(+options, canvas, defTransform);
      }
    }
  }, [options, client]);

  useEffect(() => {
    if (client !== undefined) {
      const currentTransform: Transform = client.get_transform()!;
      currentTransform.set_trans_x(x);
      currentTransform.set_trans_y(y);
      currentTransform.set_trans_z(z);
      client.set_transform(currentTransform);
    }
  }, [x, y, z]);

  return client !== undefined && options !== undefined ? (
    <div>
      <select
        className="custom-select custom-select-lg mb-3"
        value={Object.keys(wasm.RenderableOption)[options]}
        name="renderable"
        onChange={(e: React.ChangeEvent<HTMLSelectElement>) => {
          const val =
            wasm.RenderableOption[(e.target.value as unknown) as number];
          setOption(Number(val));
        }}
      >
        {Object.keys(wasm.RenderableOption).map((el) => (
          <option key={el} value={el}>
            {el}
          </option>
        ))}
      </select>
      <div>
        <div>Change X</div>
        <input
          className="form-control"
          type="number"
          value={x}
          onChange={(e) => setX(Number(e.target.value))}
        />
      </div>
      <div>
        <div>Change Y</div>
        <input
          className="form-control"
          type="number"
          value={y}
          onChange={(e) => setY(Number(e.target.value))}
        />
      </div>
      <div>
        <div>Change Z</div>
        <input
          className="form-control"
          type="number"
          value={z}
          onChange={(e) => setZ(Number(e.target.value))}
        />
      </div>
    </div>
  ) : (
    <div>Loading</div>
  );
}
