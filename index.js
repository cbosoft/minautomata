import wasmInit from "./pkg/minautomata.js";

const r = wasmInit("./pkg/minautomata_bg.wasm");

r.then(after_init).catch(console.error);

function after_init(w) {
  const r = import("./pkg/minautomata.js");
  r.then(r => after_load(r, w)).catch(console.error);
}

function after_load(rust, wasm) {
  let game = new rust.Game();
  start(game, wasm);
}

const container = document.getElementById("container");
const canvasElement = document.querySelector("canvas");
var canvas_size = 20;

function start(game, wasm) {

  canvas_size = game.get_canvas_size();

    // Get our canvas element from our index.html
    canvasElement.width = canvasElement.height = canvas_size;

    // Set up Context and ImageData on the canvas
    const canvasContext = canvasElement.getContext("2d");
    const canvasImageData = canvasContext.createImageData(
        canvasElement.width,
        canvasElement.height
    );


    //document.addEventListener("click", (ev)=>on_click(game, ev));
    canvasElement.addEventListener("mousedown", (ev)=>start_painting(game, ev));
    canvasElement.addEventListener("mouseup", stop_painting);
    canvasElement.addEventListener("mousemove", mouse_move);

    // Clear the canvas
    canvasContext.clearRect(0, 0, canvasElement.width, canvasElement.height);

    update(game, wasm, canvasImageData, canvasContext);
}


var painting = false;
var me;

function start_painting(game, e) {
  painting = true;
  me = e;
  paint(game, me);
}

function stop_painting() {
  painting = false;
}

function mouse_move(e) {
  me = e;
}


function paint(game, e) {
  me = e;
  if (painting) {
    var rect = me.target.getBoundingClientRect();
    var x = me.clientX - rect.left; //x position within the element.
    var y = me.clientY - rect.top;  //y position within the element.

    let w = window.innerWidth;
    let h = window.innerHeight;
    let s = (h < w ? h : w);
    let cs = canvas_size;
    x = Math.floor(x/s*cs);
    y = Math.floor(y/s*cs);

    game.clicked(x, y);

    setTimeout(()=> {
      paint(game, me)
    }, 30)
  }
}


function update(game, wasm, canvasImageData, canvasContext) {

    // update container size
    let w = window.innerWidth;
    let h = window.innerHeight;
    let s = (h < w ? h : w);
    container.style.width = s + "px";
    container.style.height = s + "px";

    game.update();
    const canvas_size = game.get_canvas_size();

    // Extract frame data from game obj
    const wasmByteMemoryArray = new Uint8Array(wasm.memory.buffer);
    const outputPointer = game.get_output_buffer_pointer();
    const imageDataArray = wasmByteMemoryArray.slice(
      outputPointer,
      outputPointer + canvas_size * canvas_size * 4
    );

    // console.log(imageDataArray[0, 0, 0], imageDataArray[0, 0, 1], imageDataArray[0, 0, 2], imageDataArray[0, 0, 3]);

    // Set the values to the canvas image data
    canvasImageData.data.set(imageDataArray);
    canvasContext.clearRect(0, 0, canvasElement.width, canvasElement.height);
    canvasContext.putImageData(canvasImageData, 0, 0);

    // call update again in x ms
    setTimeout(() => {
      update(game, wasm, canvasImageData, canvasContext)
    }, 10);
};