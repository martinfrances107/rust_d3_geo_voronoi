// import { Renderer } from 'benchmark';

const sizeRange = document.getElementById("size-range");
const sizeLabel = document.getElementById("size-label");
const perf = document.getElementById("perf");

import("../pkg").then((pkg) => {
  console.log("wasm is imported");

  // perf.innerHTML = 'Render Time: ...Calculating'

  // Holds elapsed samples (use to compute the standard deviation).
  let elapsedArray = [];
  // index into the elapsedArray 0..199
  let index = 0;

  // const nPoints: any = sizeRange.value;

  if (sizeRange == null) {
    return;
  }
  if (!(sizeRange instanceof HTMLInputElement)) {
    return;
  }
  const nPoints = Number(sizeRange.value);

  if (sizeLabel == null) {
    return;
  }

  // const nPoints = Number(sizeRange.inputMode);
  sizeLabel.innerText = `The number of points on the sphere: ${nPoints}`;

  const canvas = document.getElementById("c");
  if (canvas == null) {
    return;
  }

  let context;
  if (canvas instanceof HTMLCanvasElement) {
    context = canvas.getContext("2d");
  } else {
    return;
  }

  if (perf == null) {
    return;
  }

  console.log("all DOM check complete");

  const renderer = pkg.Renderer.new(nPoints);

  console.log("have renderer");

  /// TODO: Warning a function defined with a function
  const genPoints = (event) => {
    const sliderValue = Number(sizeRange.value);
    sizeLabel.innerText = `The number of points on the sphere: ${sliderValue}`;
    index = 0;
    elapsedArray = [];

    perf.innerHTML = "Render Time: ...Calculating";
    renderer.update(sliderValue);
  };
  console.log("defined geo-points");

  sizeRange.addEventListener("change", genPoints);

  var render_out;
  const renderLoop = () => {
    const t0 = performance.now();
    render_out = renderer.render();
    const t1 = performance.now();
    // Compute the mean elapsed time and compute the standard deviation based on the
    // the last 200 samples.
    const elapsed = t1 - t0;
    index = (index + 1) % 200;
    elapsedArray[index] = elapsed;
    if (index === 199) {
      const n = elapsedArray.length;
      const mean = elapsedArray.reduce((a, b) => a + b, 0) / n;
      const stdDev = Math.sqrt(
        elapsedArray.map((x) => Math.pow(x - mean, 2)).reduce((a, b) => a + b) /
          n
      );
      const meanString = mean.toPrecision(4);
      const stdDevString = stdDev.toPrecision(4);
      perf.innerHTML = `Mean Render Time: ${meanString} +/- ${stdDevString} ms`;
    }

    requestAnimationFrame(renderLoop);
  };

  renderLoop();
});
