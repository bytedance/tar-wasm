# @bytedance/tar-wasm

A faster\* experimental wasm-based tar implementation for browsers and Node.js.

\*50-160x faster, see benchmarks below.

## Usage

### Install

```bash
npm install @bytedance/tar-wasm
```

### Example

```typescript
import { TarBuilder } from "@bytedance/tar-wasm";

// Create a new tar builder
const tarBuilder = new TarBuilder();

// Optionally enable gzip compression
// tarBuilder.set_gzip(true);
// However, I recommend using the Compression Streams API instead when possible:
// https://developer.mozilla.org/en-US/docs/Web/API/Compression_Streams_API

for (const file of files) {
  // Add files to the tar
  tarBuilder.add_file(file.name, file.content);
}

// Finish building the tar and get the result
const tar: UInt8Array = tarBuilder.finish();

// Optionally convert the tar to a Blob or whatever
const tarBlob = new Blob([tar], { type: "application/x-tar" });
```

## Performance Notes

This library is optimized for performance via WebAssembly.
However, WebAssembly runs in the main thread by default and can be computationally expensive by nature depending on the size of the tar.

We recommend using [Web Worker](https://developer.mozilla.org/en-US/docs/Web/API/Web_Workers_API) or [Worker Threads](https://nodejs.org/api/worker_threads.html) to run the tar building in a separate thread.
You may want to refer to [Greenlet](https://github.com/developit/greenlet) for a lightweight implementation.

## Benchmarks

According to my benchmark, it can be 50-160x faster than existing JavaScript implementations on NPM, depending on the browser and the file size.

Results on my machine
(Intel(R) Xeon(R) Platinum 8269CY CPU @ 2.50GHz):

```
Running benchmark...
Generated 1001 random files to test in 191.13816452026367ms
@bytedance/tar-wasm: 24.62919044494629ms (baseline)
tar-js: 3943.49857711792ms (160.11482740095886x slower than @bytedance/tar-wasm)
tarts: 1251.595199584961ms (50.817553357373065x slower than @bytedance/tar-wasm)
```

See `./examples/nodejs-benchmark` folder for more details.

## TODO

- [ ] Include a Promise API working from a separate worker thread.
- [ ] Add support for reading and writing tar files.
- [x] Support of Node.js environment
- [x] Tests
- [x] Benchmarks
