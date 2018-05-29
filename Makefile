wasm:
	rustc +nightly --target wasm32-unknown-unknown -O src/wasm.rs --crate-type=cdylib

