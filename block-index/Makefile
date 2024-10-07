.PHONY: all
all:
	make build

.PHONY: build
build:
	cargo build --target wasm32-unknown-unknown --release
	substreams pack
	substreams info

.PHONY: gui
gui:
	substreams gui . -e mainnet.injective.streamingfast.io:443 index_blocks -s -1000 -t 0