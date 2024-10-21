# `Clock` Substream

[![Build Status](https://github.com/pinax-network/substreams-clock/actions/workflows/test.yml/badge.svg)](https://github.com/pinax-network/substreams-clock/actions/workflows/test.yml)
![Version](https://img.shields.io/github/v/release/pinax-network/substreams-clock)
![License](https://img.shields.io/github/license/pinax-network/substreams-clock)

> Emits `Clock` map modules at set intervals.
>
> `1m`,`3m`,`5m`,`30m`,`1h`,`2h`,`6h`,`8h`,`12h`,`1d`,`3d`,`m`,`y` are supported.

## Quickstart

```
gh repo clone pinax-network/substreams-clock
cd substreams-clock
make
make gui
```

## Releases `.spkg`

- <https://github.com/pinax-network/substreams-clock/releases>

## Mermaid graph

```mermaid
graph TD;
  store_clock[store: store_clock];
  sf.substreams.v1.Clock[source: sf.substreams.v1.Clock] --> store_clock;
  map_clock[map: map_clock];
  map_clock:params[params] --> map_clock;
  sf.substreams.v1.Clock[source: sf.substreams.v1.Clock] --> map_clock;
  store_clock -- deltas --> map_clock;
```

## Map Outputs

### `graph_out`

```json
{
  "id": "ab79f822909750f88dfb9dd0350c1ebe98d5495e9c969cdeb6e0ac993b80175b",
  "number": "6912",
  "timestamp": "2015-07-31T00:00:01Z"
}
```

### Modules

```yaml
Name: store_clock
Initial block: 0
Kind: store
Input: source: sf.substreams.v1.Clock
Value Type: string
Update Policy: set
Hash: 2810ecc8f812533b3d7d272392793a6590d863c0

Name: map_clock
Initial block: 0
Kind: map
Input: params: day
Input: source: sf.substreams.v1.Clock
Input: store: store_clock
Output Type: proto:sf.substreams.v1.Clock
Hash: 1723a789b027af657c12fe9c37eefafd56595078
```
