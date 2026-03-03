GAMES := $(patsubst games/%/,%,$(dir $(wildcard games/*/Makefile)))

.PHONY: all setup build test lint clean rename $(GAMES)

all: build

setup:
	@for game in $(GAMES); do \
		echo "Setting up $$game..."; \
		$(MAKE) -C games/$$game setup; \
	done

build:
	@mkdir -p docs/assets
	cp index.html docs/
	cp assets/icon.png docs/assets/icon.png
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

rename:
	@echo "Renaming repository to 'games'..."
	gh repo edit ollisulopuisto/zookeeper-wasm --name games
	git remote set-url origin https://github.com/ollisulopuisto/games.git
	@echo "Repository renamed. Please update any hardcoded URLs."
