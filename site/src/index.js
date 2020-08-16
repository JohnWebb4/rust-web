const js = import("rust-web/rust_web.js");

js.then((js) => {
  js.greet("WebAssembly");
});
