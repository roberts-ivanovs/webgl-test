import React, { ReactElement, useState, useEffect } from 'react';
import { GlClient, RenderableOption } from 'wasm-app';

interface Props {
  wasm: typeof import('wasm-app');
}

export default function RenderableDropdown({ wasm }: Props): ReactElement {
  const [options, setOption] = useState<RenderableOption>();
  const [client, setClient] = useState<GlClient>();

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
      client.render();
    }
  }, [options, client !== undefined]);

  return client && options !== undefined ? (
    <select
      value={Object.keys(wasm.RenderableOption)[options]}
      name="renderable"
      onChange={(e: React.ChangeEvent<HTMLSelectElement>) => {
        const val =
          wasm.RenderableOption[(e.target.value as unknown) as number];
        console.log('Selected val 2', val, Number(val));

        setOption(Number(val));
      }}
    >
      {Object.keys(wasm.RenderableOption).map((el) => (
        <option key={el} value={el}>
          {el}
        </option>
      ))}
    </select>
  ) : (
    <div>Loading</div>
  );
}
