default: build

all: test

test: build
	cargo test

build:
	soroban contract build --package escrow
	soroban contract optimize --wasm ../../target/wasm32-unknown-unknown/release/escrow.wasm  
	@ls -l ../../target/wasm32-unknown-unknown/release/*.wasm

fmt:
	cargo fmt --all

clean:
	cargo clean