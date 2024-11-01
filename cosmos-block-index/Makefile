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
	substreams gui cosmos-block-index-v0.1.0.spkg -e mainnet.injective.streamingfast.io:443 map_blocks -s 0 -t 0 --production-mode

.PHONY: gui
gui:
	substreams gui . -e mainnet.injective.streamingfast.io:443 map_blocks -s 82182057