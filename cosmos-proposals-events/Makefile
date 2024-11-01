.PHONY: all
all:
	make build


.PHONY: protogen
protogen:
	substreams protogen

.PHONY: build
build:
	cargo build --target wasm32-unknown-unknown --release
	substreams pack
	substreams info

.PHONY: gui
gui:
	substreams gui . -e mainnet.injective.streamingfast.io:443 map_events -s 8982722 -t 9157418

.PHONY: cache
cache:
	substreams gui . -e mainnet.injective.streamingfast.io:443 map_events -s 82182057 -t 0 --production-mode
