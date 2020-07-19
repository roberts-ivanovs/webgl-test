import React, { ReactElement, useState, useEffect } from 'react';
import { GlClient, RenderableOption, CanvasData, Transform } from 'wasm-app';

interface Props {
  wasm: typeof import('wasm-app');
  canvas: CanvasData;
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

  useEffect(() => {
    if (options === undefined) {
      setOption(+wasm.RenderableOption.Cube);
    } else {
      const defTransform = new wasm.Transform(0, 0, -6);
      if (client === undefined) {
        setClient(new wasm.GlClient(+options, canvas, defTransform));
      } else {
        client.set_renderable(+options, canvas, defTransform);
      }
    }
  }, [options, client]);

  useEffect(() => {
    if (client !== undefined) {
      const currentTransform = client.get_transform()!;
      currentTransform.set_trans_x(x);
      currentTransform.set_trans_y(y);
      currentTransform.set_trans_z(z);
      client.set_transform(currentTransform);
    }
  }, [
    x, y, z
  ]);

  /**
   * Perform the render call once a parameter has changed
   */
  useEffect(() => {
    if (client !== undefined) {
      client.render();
    }
  }, [options, client !== undefined, client?.get_transform()]);

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
