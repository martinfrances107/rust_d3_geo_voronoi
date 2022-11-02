import { Renderer } from 'benchmark';

const sizeRange = document.getElementById('size-range');
const sizeLabel = document.getElementById('size-label');
const perf = document.getElementById('perf');

perf.innerHTML = 'Render Time: ...Calculating'

// Holds elapsed samples (use to compute the standard deviation).
let elapsedArray = [];
// index into the elapsedArray 0..199
let index = 0;

const nPoints = sizeRange.value;
sizeLabel.innerText = `The number of points on the sphere: ${nPoints}`;

const renderer = Renderer.new(sizeRange.value);

const canvas = document.getElementById('c');
const context = canvas.getContext('2d');
const genPoints = (event) => {
    const nPoints = sizeRange.value;
    sizeLabel.innerText = `The number of points on the sphere: ${nPoints}`;
    index = 0;
    elapsedArray = [];
    perf.innerHTML = 'Render Time: ...Calculating';
    console.log('n_points', nPoints);
    console.log('renderer', renderer);
    renderer.update(nPoints);
};

sizeRange.addEventListener('change', genPoints);
const renderLoop = () => {
    context.clearRect(0, 0, 960, 600);
    const t0 = performance.now();
    renderer.render();
    const t1 = performance.now();

    // Compute the mean elapsed time and compute the standard deviation based on the
    // the last 200 samples.
    const elapsed = (t1 - t0);
    index = (index + 1) % 200;
    elapsedArray[index] = elapsed;
    if (index === 199) {
        const n = elapsedArray.length;
        const mean = elapsedArray.reduce((a, b) => a + b, 0) / n;
        const stdDev = Math.sqrt(elapsedArray.map(x => Math.pow(x - mean, 2)).reduce((a, b) => a + b) / n)
        const meanString = mean.toPrecision(4);
        const stdDevString = stdDev.toPrecision(4);
        perf.innerHTML = `Mean Render Time: ${meanString} +/- ${stdDevString} ms`;
    }

    requestAnimationFrame(renderLoop);
}

renderLoop();
