#!/usr/bin/env bash

clean:
	cargo clean
	rm -rf ./build

test: clean
	go build -o ./build/scale ./src/scale.go
	cargo test