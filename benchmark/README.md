# Benchmarks

This is a simple benchmark comparing the performance of `tar-wasm` and other
existing tar implementations.

## Note

To run this benchmark, we need to compile the package targetting `nodejs` by adding
`--target nodejs` to the `wasm-pack build` command in the `Makefile`

See: https://rustwasm.github.io/docs/wasm-pack/commands/build.html?#target

## Usage

```bash
yarn install
yarn bench
```
