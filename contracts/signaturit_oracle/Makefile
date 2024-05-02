default: build

all: test

test: build
	cargo test

build:
	soroban contract build --package signaturit_oracle
	soroban contract optimize --wasm ../../target/wasm32-unknown-unknown/release/signaturit_oracle.wasm  
	@ls -l ../../target/wasm32-unknown-unknown/release/*.wasm

fmt:
	cargo fmt --all

clean:
	cargo clean