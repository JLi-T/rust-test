import * as wasm from "wasm-gol";
import { Universe } from "wasm-gol";

const pre = document.getElementById("game-of-life-canvas");
const universe = Universe.new();

//update the pre every iteration
const renderLoop = () => {
    pre.textContent = universe.render();
    universe.tick();
  
    requestAnimationFrame(renderLoop);
};

requestAnimationFrame(renderLoop);
  