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
	substreams gui . -e mainnet.injective.streamingfast.io:443 gov_params -s 8982722 -t 9157418 --network injective

.PHONY: cache
cache:
	substreams gui . -e mainnet.injective.streamingfast.io:443 gov_params -s 82182057 -t 0 --production-mode --network injective
