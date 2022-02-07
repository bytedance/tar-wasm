# @byted/tar-wasm

An experimental wasm-based tar implementation for Node.js and browsers.

## Usage

### 🛠️ Build

```
make build
```

### 🔬 Test in Headless Browsers with `wasm-pack test`

```
wasm-pack test --headless --firefox
```

### 🎁 Publish to NPM with `wasm-pack publish`

Make sure to use `make build` to make sure scoped npm package name is correctly set.

```
wasm-pack publish
```

## About

This package is generated with [wasm-pack](https://github.com/rustwasm/wasm-pack)

## 🔋 Batteries Included

- [`wasm-bindgen`](https://github.com/rustwasm/wasm-bindgen) for communicating
  between WebAssembly and JavaScript.
- [`console_error_panic_hook`](https://github.com/rustwasm/console_error_panic_hook)
  for logging panic messages to the developer console.
- [`wee_alloc`](https://github.com/rustwasm/wee_alloc), an allocator optimized
  for small code size.
