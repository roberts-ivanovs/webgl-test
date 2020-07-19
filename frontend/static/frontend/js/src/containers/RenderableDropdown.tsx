import React, { ReactElement, useState, useEffect } from 'react';
import { GlClient, RenderableOption } from 'wasm-app';

interface Props {
  wasm: typeof import('wasm-app');
}

export default function RenderableDropdown({ wasm }: Props): ReactElement {
  const [options, setOption] = useState<RenderableOption>();
  const [client, setClient] = useState<GlClient>();
  const [x, setX] = useState(0);
  const [y, setY] = useState(0);
  const [z, setZ] = useState(-6);

  useEffect(() => {
    if (options === undefined) {
      setOption(+wasm.RenderableOption.Cube);
    } else {
      if (client === undefined) {
        setClient(new wasm.GlClient(+options));
      } else {
        client.set_renderable(+options);
      }
    }
  }, [options, client]);

  useEffect(() => {
    if (client !== undefined) {
      client.transform_xyz(x, y, z);
    }
  }, [x, y, z]);

  /**
   * Perform the render call once a parameter has changed
   */
  useEffect(() => {
    if (client !== undefined) {
      client.render();
    }
  }, [options, client !== undefined, x, y, z]);

  return client && options !== undefined ? (
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
          type="number"
          value={x}
          onChange={(e) => setX(Number(e.target.value))}
        />
      </div>
      <div>
        <div>Change Y</div>
        <input
          type="number"
          value={y}
          onChange={(e) => setY(Number(e.target.value))}
        />
      </div>
      <div>
        <div>Change Z</div>
        <input
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
