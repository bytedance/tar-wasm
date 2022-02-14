SHELL = /bin/bash
.SHELLFLAGS := -eu -o pipefail -c
.DEFAULT_GOAL := build
.DELETE_ON_ERROR:
.SUFFIXES:
.PHONY: clean test

build: pkg

pkg: src/* Cargo.toml README.md
	wasm-pack build
	sed -i -e 's/"name": "tar-wasm"/"name": "@byted\/tar-wasm"/g' pkg/package.json

test: src tests Cargo.toml
	wasm-pack test --chrome

clean: 
	rm -rf ./pkg

publish: pkg
	cd pkg; npm publish