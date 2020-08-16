const js = import("./node_modules/rust-web/rust_web.js");

js.then((js) => {
  js.greet("WebAssembly");
});
