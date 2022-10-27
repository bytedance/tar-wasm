SHELL = /bin/bash
.SHELLFLAGS := -eu -o pipefail -c
.DEFAULT_GOAL := build
.DELETE_ON_ERROR:
.SUFFIXES:
.PHONY: clean test bench

build: pkg/isomorphic

# wasm-pack outputs for one target at a time, so we need to build for each 
# target and stitch them together as a single isomorphic package
# to support node.js and web bundlers at the same time.
# see:
#   - https://rustwasm.github.io/docs/wasm-pack/commands/build.html#target
#   - https://github.com/rustwasm/wasm-pack/issues/313
pkg/isomorphic: pkg/bundler pkg/nodejs
	cp -r pkg/bundler pkg/isomorphic
	mkdir pkg/isomorphic/bundler
	mv pkg/isomorphic/tar_wasm* pkg/isomorphic/bundler
	mkdir pkg/isomorphic/nodejs
	cp pkg/nodejs/tar_wasm* pkg/isomorphic/nodejs
	cat pkg/bundler/package.json | jq '. |= .+ { "name":"@bytedance/tar-wasm", "files": [ "bundler", "nodejs" ], "module": "bundler/tar_wasm.js", "main": "nodejs/tar_wasm.js", "types": "bundler/tar_wasm.d.ts" }' > pkg/isomorphic/package.json

# build for the browser and bundlers, in ESModules format
pkg/bundler: src/* Cargo.toml README.md
	wasm-pack build --target bundler --out-dir pkg/bundler

# build for node.js, in CommonJS format
pkg/nodejs: src/* Cargo.toml README.md
	wasm-pack build --target nodejs --out-dir pkg/nodejs

test: src tests Cargo.toml
	wasm-pack test --chrome --headless

bench: pkg/isomorphic
	cd examples/nodejs-benchmark; yarn install; yarn bench;

clean:
	rm -rf ./pkg ./target ./benchmark/node_modules

publish: pkg/isomorphic
	cd pkg/isomorphic; npm publish