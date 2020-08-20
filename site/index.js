function init() {
  return WebAssembly.instantiateStreaming(fetch("./opt.wasm")).then(
    ({ instance }) => {
      const width = 500;
      const height = 500;

      const canvas = document.getElementById("screen");
      canvas.height = height;
      canvas.width = width;

      const buffer_address = instance.exports.BUFFER.value;
      const image = new ImageData(
        new Uint8ClampedArray(
          instance.exports.memory.buffer,
          buffer_address,
          4 * width * height
        ),
        width
      );

      const ctx = canvas?.getContext("2d");

      function render() {
        instance.exports.run();
        ctx.putImageData(image, 0, 0);

        requestAnimationFrame(render);
      }

      render();
    }
  );
}

init().catch(console.error);
