#!/usr/bin/env bash

clean:
	cargo clean
	rm -rf ./build

test: clean
	go build -o ./build/scale ./src/scale.go
	export CARGO_INCREMENTAL=0
	export RUSTFLAGS="-Zprofile -Ccodegen-units=1 -Copt-level=0 -Clink-dead-code -Coverflow-checks=off -Zpanic_abort_tests -Cpanic=abort"
	export RUSTDOCFLAGS="-Cpanic=abort"
	cargo build
	cargo test -- -Z unstable-options --format json --report-time | tee report/test.json
	go run ./report/web_generator.go report/test.json report/web/index.html
