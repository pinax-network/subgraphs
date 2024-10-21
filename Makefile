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

.PHONY: gui
gui:
	substreams gui substreams.yaml -e eth.substreams.pinax.network:443 map_clock -s 0 -t 200000 -H "X-Sf-Substreams-Parallel-Jobs: 100" --production-mode

.PHONY: cache
cache:
	substreams-sink-noop eth.substreams.pinax.network:443 substreams.yaml map_clock 0: -H "X-Sf-Substreams-Parallel-Jobs: 100"
