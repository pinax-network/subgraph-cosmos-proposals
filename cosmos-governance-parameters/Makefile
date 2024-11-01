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
	substreams gui . -e mainnet.injective.streamingfast.io:443 graph_out -s 8982722 -t 9157418

.PHONY: cache
cache:
	substreams gui . -e mainnet.injective.streamingfast.io:443 graph_out -s 82182057 -t 0 --production-mode
