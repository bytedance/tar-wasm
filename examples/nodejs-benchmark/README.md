# Benchmarks

This is a simple benchmark comparing the performance of `tar-wasm` and other
existing tar implementations.

## Note

To run this benchmark, we need to compile the package targeting `nodejs` as it is easier to test in Node.js.
The performance should be similar in both Node.js and Chrome as they're both using v8 engines.

See: https://rustwasm.github.io/docs/wasm-pack/commands/build.html?#target

## Usage

Run the `make bench` command in the project root to build and run the benchmark.

To run without build: `yarn bench`
