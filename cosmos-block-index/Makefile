.PHONY: all
all:
	make build

.PHONY: build
build:
	cargo build --target wasm32-unknown-unknown --release
	substreams pack
	substreams info
	substreams graph

.PHONY: cache
cache:
	substreams gui . -e mainnet.injective.streamingfast.io:443 map_blocks -s 0 -t 0 --production-mode --network injective

.PHONY: gui
gui:
	substreams gui . -e mainnet.injective.streamingfast.io:443 map_blocks -s 82182057