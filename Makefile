.PHONY: all
all:
	make build
	make pack
	make graph
	make info

.PHONY: build
build:
	cargo build --target wasm32-unknown-unknown --release

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
	substreams run -e eth.substreams.pinax.network:9000 graph_out -s -1

.PHONY: gui
gui:
	substreams gui -e eth.substreams.pinax.network:9000 graph_out -s -1

.PHONY: deploy
deploy:
	graph build
	graph deploy --studio clock