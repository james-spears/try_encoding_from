# Binary Tree Network (btree_network)

[![Build Status](https://travis-ci.com/jameone/btree_network.svg?branch=main)](https://travis-ci.com/jameone/btree_network)
[![Code Version](https://img.shields.io/crates/v/btree_network)](https://img.shields.io/crates/v/btree_network)
[![Docs badge]][docs.rs]

[Docs badge]: https://img.shields.io/badge/docs.rs-rustdoc-green
[docs.rs]: https://docs.rs/btree_network/

This library is a minimal implementation of a network 
(abstract data structure) by way of a single binary tree map
(`BTreeMap`). This implementation is often referred to as
an adjacency list.

The primary goals of this implementation are to be 
minimal and idiomatic to the Rust language. The `alloc`
crate is the only dependency when compiled with default
features and is not optional. As one might assume, `alloc`
is required for reason the implementation relies on `BTreeMap`
(and the `BTreeSet` wrapper).

Secondary concerns include serialization,
deserialization, and encoding. For these the `serde`,
`serde_json`, `serde_yaml`, and `serde_cbor` crates
are included and available under the feature flags:
`serde`, `serde_json`, `serde_yaml`, and `serde_cbor`.
Please see the encoding module's [API](../try_encoding_from/src/encoding/api.rs)
for the available optional trait definitions. *Note: using
`serde_json`, `serde_yaml`, or `serde_cbor` features will
require inclusion of the `serde` feature, else the library
will not compile.*

## Example
```rust
use btree_network::BTreeNetwork;

fn main() {
    let mut network: BTreeNetwork<String, String> = BTreeNetwork::new();
    // Add nodes.
    network.add_vertex(String::from("Tarzan"));
    network.add_vertex(String::from("Jane"));
    // Add a relationship.
    network.add_edge(String::from("Tarzan"), String::from("Jane"));
    
    // Assert relationship now exists.
    assert!(network.adjacdent(String::from("Tarzan"), String::from("Jane")));
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