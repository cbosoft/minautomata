# MINAUTOMATA

Falling sand game in rust for the browser.

# Live demo

[cmjb.tech/minautomata](https://cmjb.tech/minautomata)

# Building

Building MINAUTOMATA requires [`wasm-bindgen`](https://rustwasm.github.io/wasm-bindgen/) and [`wasm-pack`](https://rustwasm.github.io/docs/wasm-pack/). Build with:
```bash
wasm-pack build --target web
```

Then serve the repo root directory with e.g.:
```bash
python -m http.server
```

Open up the hosted address and then you should be presented with the game!