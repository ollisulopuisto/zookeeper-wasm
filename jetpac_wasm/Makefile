.PHONY: build serve test lint

build:
	@echo "Building Jetpac WASM..."
	cargo build --target wasm32-unknown-unknown --release
	@mkdir -p ../docs
	cp ../target/wasm32-unknown-unknown/release/jetpac_wasm.wasm ../docs/
	cp index.html ../docs/
	@if [ ! -f ../docs/mq_js_bundle.js ]; then 
		curl -sO https://not-fl3.github.io/miniquad-samples/mq_js_bundle.js && mv mq_js_bundle.js ../docs/; 
	fi

serve: build
	@echo "Serving Jetpac WASM at http://localhost:8000"
	python3 -m http.server -d ../docs

test:
	@echo "Running tests..."
	cargo test

lint:
	@echo "Running clippy..."
	cargo clippy -- -D warnings
