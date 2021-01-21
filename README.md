# Try\<Encoding\>From  (try_encoding_from)

[![Build Status](https://travis-ci.com/jameone/try_encoding_from.svg?branch=main)](https://travis-ci.com/jameone/try_encoding_from)
[![Code Version](https://img.shields.io/crates/v/try_encoding_from)](https://img.shields.io/crates/v/try_encoding_from)
[![Docs badge]][docs.rs]

[Docs badge]: https://img.shields.io/badge/docs.rs-rustdoc-green
[docs.rs]: https://docs.rs/try_encoding_from/

This library contains a convenient API wrapper for `serde_json`,
`serde_yaml`, and `serde_cbor` miming.

The `serde`, `serde_json`, `serde_yaml`, and `serde_cbor` crates
are included and available under the feature flags:
`json`, `yaml`, and `cbor`.
Please see the encoding module's [API](./src/encoding/api.rs)
for the available optional trait definitions. *Note: using
`json`, `yaml`, or `cbor` features will
require inclusion of the `serde` crate as Serialize, Deserialize,
DeserializeOwned traits are required for the library to compile.*

## Example
```rust
use try_encoding_from::TryJsonFrom;
use btree_graph::{BTreeGraph, AddVertex, AddEdge, Vertices};

fn main() {
    // Add three nodes.
    let mut graph: BTreeGraph<usize, usize> = BTreeGraph::new();
    graph.add_vertex(0);
    graph.add_vertex(1);
    graph.add_vertex(2);

    // Check there is indeed three nodes.
    assert_eq!(graph.vertices().len(), 3);

    // Add an edge (0, 1) = 2, (1, 2) = 3, and (0, 2) = 4.
    graph.add_edge(0, 1, 2).unwrap();
    graph.add_edge(1, 2, 3).unwrap();
    graph.add_edge(0, 2, 4).unwrap();

    let json_string: String = String::try_json_from(graph)?;
    assert_eq!(json_string, String::from("{\"vertices\":{\"0\":[2,4],\"1\":[3],\"2\":[]},\"edges\":{\"2\":[0,1],\"3\":[1,2],\"4\":[0,2]}}"));
}
```

## Usage

Add the following to your `Cargo.toml` file:
```toml
[dependencies]
try_encoding_from = "0.1.0"
```

## API

Please see the [API](src/encoding/api.rs) for a full list of
available methods.

## License

This work is dually licensed under MIT OR Apache-2.0.