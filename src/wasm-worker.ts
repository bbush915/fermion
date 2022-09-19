import * as Comlink from "comlink";

import init, { initThreadPool, Scene } from "../crate/pkg/fermion-wasm";

export interface IWASM {
  initialize: typeof initialize;
  render: typeof render;
}

async function initialize() {
  await init();
  await initThreadPool(navigator.hardwareConcurrency);
}

function render(input: any, concurrency: number) {
  const scene = new Scene(input);
  const renderContext = scene.render(concurrency);

  return Comlink.proxy(renderContext);
}

Comlink.expose({
  initialize,
  render,
});
