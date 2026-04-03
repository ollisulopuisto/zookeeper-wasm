VERSION=26.3.300000
WASM_BINDGEN=../../../bin/wasm-bindgen
WASM_OPT=wasm-opt

.PHONY: build serve

build:
	@echo "Building VoxelDash WASM..."
	cargo build --target wasm32-unknown-unknown --release
	@mkdir -p dist/pkg
	@if [ -f ../../target/wasm32-unknown-unknown/release/voxeldash.wasm ]; then \
		$(WASM_BINDGEN) --target web --out-dir dist/pkg ../../target/wasm32-unknown-unknown/release/voxeldash.wasm; \
	elif [ -f ../../target/wasm32-unknown-unknown/release/voxeldash_bg.wasm ]; then \
		$(WASM_BINDGEN) --target web --out-dir dist/pkg ../../target/wasm32-unknown-unknown/release/voxeldash.wasm; \
	else \
		echo "Error: WASM binary not found in target directory"; exit 1; \
	fi
	cp index.html dist/
	@if command -v $(WASM_OPT) >/dev/null 2>&1; then \
		$(WASM_OPT) -Oz dist/pkg/voxeldash_bg.wasm -o dist/pkg/voxeldash_bg.wasm; \
	fi

serve: build
	@echo "Serving VoxelDash at http://0.0.0.0:23232 (accessible on LAN)"
	python3 -m http.server -d dist 23232
