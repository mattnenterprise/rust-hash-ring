extern crate hash_ring;

use hash_ring::HashRing;
use hash_ring::NodeInfo;
use std::hash::BuildHasherDefault;
use std::hash::Hasher;

// This is a hasher that always returns the same number
// no matter the input. This is meant as an example and
// should never be used in production code as all keys go
// to the same node.
#[derive(Default)]
struct ConstantHasher;

impl Hasher for ConstantHasher {
    fn write(&mut self, _bytes: &[u8]) {
        // Do nothing
    }

    fn finish(&self) -> u64 {
        return 1;
    }
}

type ConstantBuildHasher = BuildHasherDefault<ConstantHasher>;

fn main() {
    let mut nodes: Vec<NodeInfo> = Vec::new();
    nodes.push(NodeInfo {
        host: "localhost",
        port: 15324,
    });
    nodes.push(NodeInfo {
        host: "localhost",
        port: 15325,
    });
    nodes.push(NodeInfo {
        host: "localhost",
        port: 15326,
    });
    nodes.push(NodeInfo {
        host: "localhost",
        port: 15327,
    });
    nodes.push(NodeInfo {
        host: "localhost",
        port: 15328,
    });
    nodes.push(NodeInfo {
        host: "localhost",
        port: 15329,
    });

    let hash_ring: HashRing<NodeInfo, ConstantBuildHasher> =
        HashRing::with_hasher(nodes, 10, ConstantBuildHasher::default());

    println!(
        "Key: '{}', Node: {}",
        "hello",
        hash_ring.get_node(("hello").to_string()).unwrap()
    );
    println!(
        "Key: '{}', Node: {}",
        "dude",
        hash_ring.get_node(("dude").to_string()).unwrap()
    );
    println!(
        "Key: '{}', Node: {}",
        "martian",
        hash_ring.get_node(("martian").to_string()).unwrap()
    );
    println!(
        "Key: '{}', Node: {}",
        "tardis",
        hash_ring.get_node(("tardis").to_string()).unwrap()
    );
}
