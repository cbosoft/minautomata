import wasmInit from "./pkg/minautomata.js";

const runWasm = async () => {
  // Instantiate our wasm module
  const rustWasm = await wasmInit("./pkg/minautomata_bg.wasm");

  // Run init
  rustWasm.init_game();

  // Create a Uint8Array to give us access to Wasm Memory
  // const wasmByteMemoryArray = new Uint8Array(rustWasm.memory.buffer);

  // Get our canvas element from our index.html
  const canvasElement = document.querySelector("canvas");
  canvasElement.width = 20; //rustWasm.get_canvas_size();
  canvasElement.height = 20; //rustWasm.get_canvas_size();

  const container = document.getElementById("container");

  // Set up Context and ImageData on the canvas
  const canvasContext = canvasElement.getContext("2d");
  const canvasImageData = canvasContext.createImageData(
    canvasElement.width,
    canvasElement.height
  );

  // Clear the canvas
  canvasContext.clearRect(0, 0, canvasElement.width, canvasElement.height);

  const drawCheckerBoard = () => {
    const canvas_size = 20;

    let w = window.innerWidth;
    let h = window.innerHeight;
    let s = (h < w ? h : w);
    container.style.width = s + "px";
    container.style.height = s + "px";
    // console.log(s);

    // Generate a new checkboard in wasm
    rustWasm.update();

    // Pull out the RGBA values from Wasm memory
    // Starting at the memory index of out output buffer (given by our pointer)
    // 20 * 20 * 4 = checkboard max X * checkerboard max Y * number of pixel properties (R,G.B,A)

    const wasmByteMemoryArray = new Uint8Array(rustWasm.memory.buffer);
    const outputPointer = rustWasm.get_output_buffer_pointer();
    const imageDataArray = wasmByteMemoryArray.slice(
      outputPointer,
      outputPointer + canvas_size * canvas_size * 4
    );

    //console.log(imageDataArray[0, 0, 0]);

    // Set the values to the canvas image data
    canvasImageData.data.set(imageDataArray);

    // Clear the canvas
    canvasContext.clearRect(0, 0, canvasElement.width, canvasElement.height);

    // Place the new generated checkerboard onto the canvas
    canvasContext.putImageData(canvasImageData, 0, 0);
  };

  // call func once
  // drawCheckerBoard();

  // call it every 100 ms from now on
  setInterval(() => {
    drawCheckerBoard();
  }, 1000);
};

runWasm();