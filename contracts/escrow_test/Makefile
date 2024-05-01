default: build

all: test

test: build
	cargo test

build:
	soroban contract build --package escrow_test
	soroban contract optimize --wasm ../../target/wasm32-unknown-unknown/release/escrow_test.wasm  
	@ls -l ../../target/wasm32-unknown-unknown/release/*.wasm

fmt:
	cargo fmt --all

clean:
	cargo clean