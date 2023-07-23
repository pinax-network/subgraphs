.PHONY: all
all:
	make build
	make pack
	make graph
	make info

.PHONY: build
build:
	cargo build --target wasm32-unknown-unknown --release
	graph build

.PHONY: pack
pack:
	substreams pack

.PHONY: graph
graph:
	substreams graph

.PHONY: info
info:
	substreams info

.PHONY: run
run:
	substreams run -e eth.firehose.pinax.network:9000 graph_out -s -1

.PHONY: gui
gui:
	substreams gui -e eth.firehose.pinax.network:9000 graph_out -s -1

.PHONY: deploy
deploy:
	graph deploy --studio clock