.PHONY: all
all:
	make build

.PHONY: build
build:
	graph build

.PHONY: deploy
deploy:
	graph build
	graph create --node=http://localhost:8020 cosmoshub_proposals
	graph deploy --node=http://localhost:8020 cosmoshub_proposals

.PHONY: publish
publish:
	graph build
	graph publish --subgraph-id ??

.PHONY: gui
gui:
	substreams gui ../../substreams.yaml -e mainnet.cosmos.streamingfast.io:443  graph_out -s -1000