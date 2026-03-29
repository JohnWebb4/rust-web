import initWasm from "./pkg/rust_web.js";

function init() {
  let config = {
    env: {
      console_log: (arg) => console.log("rust", arg),
    },
  };

  return initWasm(config).then((instance) => {
    const width = 900;
    const height = 350;

    const canvas = document.getElementById("screen");
    canvas.height = height;
    canvas.width = width;

    const buffer_address = instance.buffer_ptr();
    const image = new ImageData(
      new Uint8ClampedArray(
        instance.memory.buffer,
        buffer_address,
        4 * width * height,
      ),
      width,
    );

    const ctx = canvas?.getContext("2d");

    function render() {
      instance.run();
      ctx.putImageData(image, 0, 0);

      requestAnimationFrame(render);
    }

    render();
  });
}

init().catch(console.error);
