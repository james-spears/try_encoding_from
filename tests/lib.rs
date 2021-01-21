extern crate alloc;
#[cfg(feature = "serde_cbor")]
pub use try_encoding_from::{TryCborFrom, TryCborInto};
#[cfg(feature = "serde_json")]
pub use try_encoding_from::{TryJsonFrom, TryJsonInto};
#[cfg(feature = "serde_yaml")]
pub use try_encoding_from::{TryYamlFrom, TryYamlInto};
#[cfg(any(feature = "serde_json", feature = "serde_yaml", feature = "serde_cbor"))]
use try_encoding_from::Error;
#[cfg(any(feature = "serde_json", feature = "serde_yaml", feature = "serde_cbor"))]
use btree_graph::{BTreeGraph, AddEdge, AddVertex, Vertices};
#[cfg(any(feature = "serde_json", feature = "serde_yaml", feature = "serde_cbor"))]
use alloc::string::String;
#[cfg(any(feature = "serde_json", feature = "serde_yaml", feature = "serde_cbor"))]
use alloc::vec::Vec;

#[test]
#[cfg(all(feature = "serde", feature = "serde_json"))]
fn test_json_encoding() -> Result<(), Error> {
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

    let exp_json_string: String = String::from("{\"vertices\":{\"0\":[2,4],\"1\":[3],\"2\":[]},\"edges\":{\"2\":[0,1],\"3\":[1,2],\"4\":[0,2]}}");
    let json_string: String = String::try_json_from(graph.clone())?;
    assert_eq!(exp_json_string, json_string);

    let decoded_graph: BTreeGraph<usize, usize> = json_string.try_json_into()?;
    assert_eq!(decoded_graph, graph.clone());

    // Vec<u8>
    let graph: BTreeGraph<usize, usize> = BTreeGraph::new();
    let bytes: Vec<u8> = Vec::try_json_from(graph.clone())?;
    let decoded_graph: BTreeGraph<usize, usize> = bytes.try_json_into()?;
    assert_eq!(decoded_graph, graph);

    Ok(())
}

#[test]
#[cfg(all(feature = "serde", feature = "serde_cbor"))]
fn test_cbor_encoding() -> Result<(), Error> {
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

    let bytes: Vec<u8> = Vec::try_cbor_from(graph.clone())?;
    assert_eq!(bytes.as_slice().len(), 39);

    let decoded_graph: BTreeGraph<usize, usize> = bytes.try_cbor_into()?;
    assert_eq!(decoded_graph, graph);
    Ok(())
}

#[test]
#[cfg(all(feature = "serde", feature = "serde_yaml"))]
fn test_try_from_yaml() -> Result<(), Error> {
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

    let exp_yaml_string: String = String::from("---\nvertices:\n  0:\n    - 2\n    - 4\n  1:\n    - 3\n  2: []\nedges:\n  2:\n    - 0\n    - 1\n  3:\n    - 1\n    - 2\n  4:\n    - 0\n    - 2");
    let yaml_string: String = String::try_yaml_from(graph.clone())?;
    assert_eq!(exp_yaml_string, yaml_string);

    let decoded_graph: BTreeGraph<usize, usize> = yaml_string.try_yaml_into()?;
    assert_eq!(decoded_graph, graph.clone());

    // Vec<u8>
    let graph: BTreeGraph<usize, usize> = BTreeGraph::new();
    let bytes: Vec<u8> = Vec::try_yaml_from(graph.clone())?;
    let decoded_graph: BTreeGraph<usize, usize> = bytes.try_yaml_into()?;
    assert_eq!(decoded_graph, graph);
    Ok(())
}