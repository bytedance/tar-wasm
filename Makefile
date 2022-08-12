SHELL = /bin/bash
.SHELLFLAGS := -eu -o pipefail -c
.DEFAULT_GOAL := build
.DELETE_ON_ERROR:
.SUFFIXES:
.PHONY: clean test benchmark

build: pkg/bundler

# wasm-pack targets bundlers by default
# https://rustwasm.github.io/docs/wasm-pack/commands/build.html#target

pkg/bundler: src/* Cargo.toml README.md
	wasm-pack build --target bundler --out-dir pkg/bundler

test: src tests Cargo.toml
	wasm-pack test --chrome --headless

# for benchmarking only
pkg/nodejs: src/* Cargo.toml README.md
	wasm-pack build --target nodejs --out-dir pkg/nodejs

clean: 
	rm -rf ./pkg ./target ./benchmark/node_modules


bench: pkg/nodejs
	cd benchmark; yarn install; yarn bench;

# updating the package name with sed before publish
# because wasm-pack doesn't support '@' in the Cargo.toml package name
publish: pkg/bundler
	sed -i -e 's/"name": "tar-wasm"/"name": "@bytedance\/tar-wasm"/g' pkg/*/package.json
	cd pkg/bundler; npm publish