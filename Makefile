.PHONY: build
build:
	cargo build --target wasm32-unknown-unknown --release

.PHONY: package
package: build
	substreams pack ./substreams.yaml

.PHONY: gui
gui:
	substreams gui . -e mainnet.injective.streamingfast.io:443 graph_out -s 8982722 -t 8982732 --debug-modules-output graph_out

.PHONY: run
run:
	substreams run . -e mainnet.injective.streamingfast.io:443 graph_out -s 8982722 -t 8982732