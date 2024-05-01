default: build

all: test

test: build
	cargo test

build:
	soroban contract build --package notes_nft
	soroban contract optimize --wasm ../../target/wasm32-unknown-unknown/release/notes_nft.wasm  
	@ls -l ../../target/wasm32-unknown-unknown/release/*.wasm

fmt:
	cargo fmt --all

clean:
	cargo clean