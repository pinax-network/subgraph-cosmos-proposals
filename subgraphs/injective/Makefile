.PHONY: all
all:
	make build

.PHONY: build
build:
	graph build

.PHONY: deploy
deploy:
	graph build
	graph create --node=http://localhost:8020 injective_proposals
	graph deploy --node=http://localhost:8020 injective_proposals

.PHONY: publish
publish:
	graph build
	graph publish --subgraph-id ??

.PHONY: gui
gui:
	substreams gui . -e mainnet.injective.streamingfast.io:443 graph_out -s 82182057 --debug-modules-output graph_out