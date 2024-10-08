.PHONY: all
all:
	make build

.PHONY: build
build:
	cargo build --target wasm32-unknown-unknown --release
	substreams pack
	substreams info

.PHONY: cache
cache:
	substreams gui . -e mainnet.injective.streamingfast.io:443 map_blocks -s 82182057 -t 0 --production-mode

.PHONY: gui
gui:
	substreams gui . -e mainnet.injective.streamingfast.io:443 map_blocks -s 82182057