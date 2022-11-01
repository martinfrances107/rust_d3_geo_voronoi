import { Renderer } from "benchmark";

let renderer = Renderer.new(505);

renderer.render();
const renderLoop = () => {
    renderer.render();

    requestAnimationFrame(renderLoop);

}

renderLoop();
