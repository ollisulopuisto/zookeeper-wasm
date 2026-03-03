GAMES := $(patsubst games/%/,%,$(dir $(wildcard games/*/Makefile)))

.PHONY: all setup build test lint $(GAMES)

all: build

setup:
	@for game in $(GAMES); do \
		echo "Setting up $$game..."; \
		$(MAKE) -C games/$$game setup; \
	done

build:
	@mkdir -p docs
	cp index.html docs/
	@for game in $(GAMES); do \
		echo "Building $$game..."; \
		$(MAKE) -C games/$$game build; \
		mkdir -p docs/$$game; \
		cp -r games/$$game/dist/* docs/$$game/; \
	done

test:
	cargo test --workspace
	@for game in $(GAMES); do \
		if [ -f games/$$game/Makefile ]; then \
			$(MAKE) -C games/$$game test; \
		fi; \
	done

lint:
	@for game in $(GAMES); do \
		$(MAKE) -C games/$$game lint; \
	done

clean:
	cargo clean
	@for game in $(GAMES); do \
		rm -rf games/$$game/dist; \
	done
	rm -rf docs
