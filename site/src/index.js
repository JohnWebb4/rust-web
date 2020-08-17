import("rust-web/rust_web_bg.wasm").then((module) => {
  console.log("module", module);
  let width = 500;
  let height = 500;

  let canvas = document.getElementById("screen");
  if (canvas.getContext) {
    let context = canvas.getContext("2d");

    let byteSize = width * height * 4;
    let pointer = module.alloc(byteSize);

    let usub = new Uint8ClampedArray(module.memory.buffer, pointer, byteSize);

    let img = new ImageData(usub, width, height);

    let start = null;

    function step(timestamp) {
      let progress;

      if (start == null) start = timestamp;
      progress = timestamp - start;
      if (progress > 100) {
        module.fill(pointer, width, height, timestamp);

        start = timestamp;

        window.requestAnimationFrame(draw);
      } else {
        window.requestAnimationFrame(step);
      }
    }

    function draw() {
      context.putImageData(img, 0, 0);
      window.requestAnimationFrame(step);
    }

    window.requestAnimationFrame(step);
  }
});
