.PHONY: build serve test lint

build:
	@echo "Building Jetpac WASM..."
	cargo build --target wasm32-unknown-unknown --release
	@mkdir -p dist
	@if [ -f ../../target/wasm32-unknown-unknown/release/jetpac.wasm ]; then \
		cp ../../target/wasm32-unknown-unknown/release/jetpac.wasm dist/jetpac.wasm; \
	else \
		echo "Error: WASM binary not found in target directory"; exit 1; \
	fi
	cp index.html dist/
	@if [ ! -f dist/mq_js_bundle.js ]; then \
		curl -sO https://not-fl3.github.io/miniquad-samples/mq_js_bundle.js && mv mq_js_bundle.js dist/; \
	fi


serve: build
	@echo "Serving Jetpac WASM at http://localhost:8000"
	python3 -m http.server -d dist

test:
	@echo "Running tests..."
	cargo test

lint:
	@echo "Running clippy..."
	cargo clippy -- -D warnings
