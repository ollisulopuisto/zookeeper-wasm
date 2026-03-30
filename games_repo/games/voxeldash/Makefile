.PHONY: build serve clean lint

WASM_BINDGEN := ../../../bin/wasm-bindgen

build:
	@echo "Building VoxelDash WASM..."
	RUSTUP_HOME=$(shell pwd)/../../../.rustup_local CARGO_HOME=$(shell pwd)/../../../.cargo_local cargo build --target wasm32-unknown-unknown --release
	@mkdir -p dist
	@if [ -f ../../target/wasm32-unknown-unknown/release/voxeldash.wasm ]; then \
		$(WASM_BINDGEN) --target web --out-dir dist/pkg ../../target/wasm32-unknown-unknown/release/voxeldash.wasm; \
	else \
		echo "Error: WASM binary not found in target directory"; exit 1; \
	fi
	cp index.html dist/

serve: build
	@echo "Serving VoxelDash at http://0.0.0.0:8000 (accessible on LAN)"
	python3 -m http.server 8000 -d dist

clean:
	rm -rf dist
	cargo clean

lint:
	cargo clippy -- -D warnings
