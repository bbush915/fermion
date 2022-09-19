import * as Comlink from "comlink";

import DefaultInput from "./assets/default-input.json";
import "./index.css";
import { IWASM } from "./wasm-worker";

// NOTE - Initialization

const UPDATE_INTERVAL = 1000 / 15;

const inputEl = document.getElementById("input") as HTMLTextAreaElement;
inputEl.value = JSON.stringify(DefaultInput, null, 2);

const startEl = document.getElementById("start") as HTMLButtonElement;

const concurrencyLabelEl = document.getElementById(
  "concurrency-label"
) as HTMLLabelElement;
const concurrencyInputEl = document.getElementById(
  "concurrency-input"
) as HTMLInputElement;
concurrencyInputEl.max = String(navigator.hardwareConcurrency);
concurrencyInputEl.oninput = function (e: Event) {
  const target = e.target as HTMLInputElement;
  concurrencyLabelEl.innerText = `Concurrency: ${target.value}`;
};

const progressEl = document.getElementById("progress") as HTMLDivElement;

const outputEl = document.getElementById("output") as HTMLCanvasElement;

const downloadEl = document.getElementById("download") as HTMLButtonElement;
downloadEl.onclick = function () {
  const link = document.createElement("a");

  link.download = "fermion_out.png";
  link.href = outputEl.toDataURL();

  link.click();
};

(async function initializeWasm() {
  const wasm = Comlink.wrap<IWASM>(
    new Worker(new URL("./wasm-worker.ts", import.meta.url), {
      type: "module",
    })
  );

  await wasm.initialize();

  startEl.onclick = async function start() {
    let input: any;
    const concurrency = Number(concurrencyInputEl.value);

    try {
      input = JSON.parse(inputEl.value);

      if (input.width > 650 || input.height > 650) {
        throw new Error("Width and height must not exceed 650");
      }

      outputEl.width = input.width;
      outputEl.height = input.height;
    } catch (e) {
      alert(`Invalid JSON input: ${e}`);
      return;
    }

    startEl.disabled = true;
    concurrencyInputEl.disabled = true;

    const renderContext = await wasm.render(input, concurrency);
    new RenderState(renderContext).start();
  };
})();

class RenderState {
  private readonly context: any;

  private startTime: number = 0;
  private intervalId: number = 0;

  constructor(context: any) {
    this.context = context;
  }

  start() {
    this.startTime = Date.now();

    this.intervalId = setInterval(
      this.update.bind(this),
      UPDATE_INTERVAL
    ) as any;

    this.context.getPromise().then((imageData: ImageData) => {
      clearInterval(this.intervalId);

      this.updateProgress(1);
      this.updateOutput(imageData);

      concurrencyInputEl.disabled = false;
      startEl.disabled = false;
    });
  }

  private async update() {
    const progress = await this.context.getCurrentProgress();
    const imageData = await this.context.getCurrentImageData();

    this.updateProgress(progress);
    this.updateOutput(imageData);
  }

  private updateProgress(progress: number) {
    const elapsedTime = Date.now() - this.startTime;

    progressEl.style.width = `${progress * 100}%`;
    progressEl.innerText = `${Math.round(progress * 10000) / 100}% (${
      elapsedTime / 1000
    }s)`;
  }

  private updateOutput(data: ImageData) {
    const ctx = outputEl.getContext("2d") as CanvasRenderingContext2D;
    ctx.putImageData(data, 0, 0);
  }
}
