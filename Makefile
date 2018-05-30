wasm:
	rustc +nightly --target wasm32-unknown-unknown -O src/wasm.rs -o frontend/static/wasm.wasm --crate-type=cdylib
runserver:
	python3 -m http.server
