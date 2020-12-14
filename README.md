rust-hash-ring
================

Consistent Hashing library for Rust

[![Crates.io](https://img.shields.io/crates/d/hash_ring.svg)](https://crates.io/crates/hash_ring)
[![crates.io](https://img.shields.io/crates/v/hash_ring.svg)](https://crates.io/crates/hash_ring)
[![Crates.io](https://img.shields.io/crates/l/hash_ring.svg)](https://crates.io/crates/hash_ring)
[![CI](https://github.com/mattnenterprise/rust-hash-ring/workflows/CI/badge.svg)](https://github.com/mattnenterprise/rust-hash-ring/actions?query=workflow%3ACI)
[![Coverage Status](https://codecov.io/gh/mattnenterprise/rust-hash-ring/branch/master/graph/badge.svg)](https://app.codecov.io/gh/mattnenterprise/rust-hash-ring/branch/master)

[Documentation](https://docs.rs/hash_ring)

### Usage
```rust
extern crate hash_ring;

use hash_ring::HashRing;
use hash_ring::NodeInfo;

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

    let mut hash_ring: HashRing<NodeInfo> = HashRing::new(nodes, 10);

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

    hash_ring.remove_node(&NodeInfo {
        host: "localhost",
        port: 15329,
    });

    println!(
        "Key: '{}', Node: {}",
        "hello",
        hash_ring.get_node(("hello").to_string()).unwrap()
    );

    hash_ring.add_node(&NodeInfo {
        host: "localhost",
        port: 15329,
    });

    println!(
        "Key: '{}', Node: {}",
        "hello",
        hash_ring.get_node(("hello").to_string()).unwrap()
    );
}
```

For an example of how to use a custom hash function you can look at [examples/custom_hasher.rs](https://github.com/mattnenterprise/rust-hash-ring/blob/master/examples/custom_hasher.rs)

### Contributing
 Just fork it, implement your changes and submit a pull request.

### License

MIT
