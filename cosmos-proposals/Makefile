.PHONY: all
all:
	make build

.PHONY: build
build:
	cargo build --target wasm32-unknown-unknown --release
	substreams pack
	substreams info
	substreams graph

.PHONY: gui
gui:
	substreams gui . -e mainnet.injective.streamingfast.io:443 graph_out -s 8982722 -t 9157418 --network injective

.PHONY: cache
cache:
	substreams gui . -e mainnet.injective.streamingfast.io:443 graph_out -s 82182057 -t 0 --production-mode --network injective

.PHONY: run
run:
	substreams run . -e mainnet.injective.streamingfast.io:443 graph_out -s 8982722 -t 8982732 --network injective