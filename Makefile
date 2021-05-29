#!/usr/bin/env bash

clean:
	cargo clean
	rm -rf ./build

test: clean
	go build -o ./build/scale ./src/scale.go
	cargo test --no-fail-fast -- -Z unstable-options --format json --report-time | tee report/test.json
	go run ./report/web_generator.go report/test.json report/web/index.html
