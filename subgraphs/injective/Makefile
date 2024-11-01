.PHONY: all
all:
	make build

.PHONY: build
build:
	substreams pack
	substreams info
	graph build

.PHONY: deploy
deploy:
	graph build
	graph create --node=http://localhost:8020 injective_proposals
	graph deploy --node=http://localhost:8020 injective_proposals

.PHONY: publish
publish:
	graph build
	graph publish --subgraph-id 2aYHh1GtHqHTU782VMxg5Hzpzsc4q4WdxniKW7MAvBBj

.PHONY: gui
gui:
	substreams gui ../../substreams.yaml -e mainnet.injective.streamingfast.io:443 graph_out -s 39438554