rust-hash-ring
================

Consistent Hashing library for Rust

[![Crates.io](https://img.shields.io/crates/d/hash_ring.svg)](https://crates.io/crates/hash_ring)
[![crates.io](https://img.shields.io/crates/v/hash_ring.svg)](https://crates.io/crates/hash_ring)
[![Crates.io](https://img.shields.io/crates/l/hash_ring.svg)](https://crates.io/crates/hash_ring)
[![Build Status](https://travis-ci.org/mattnenterprise/rust-hash-ring.svg)](https://travis-ci.org/mattnenterprise/rust-hash-ring)
[![Build Status](https://ci.appveyor.com/api/projects/status/github/mattnenterprise/rust-hash-ring?svg=true)](https://ci.appveyor.com/api/projects/status/github/mattnenterprise/rust-hash-ring)
[![Coverage Status](https://coveralls.io/repos/github/mattnenterprise/rust-hash-ring/badge.svg?branch=master)](https://coveralls.io/github/mattnenterprise/rust-hash-ring?branch=master)

[Documentation](https://docs.rs/hash_ring)

### Usage
```rust
extern crate hash_ring;

use hash_ring::HashRing;
use hash_ring::NodeInfo;

fn main() {
    let mut nodes: Vec<NodeInfo> = Vec::new();
    nodes.push(NodeInfo{host: "localhost", port: 15324});
    nodes.push(NodeInfo{host: "localhost", port: 15325});
    nodes.push(NodeInfo{host: "localhost", port: 15326});
    nodes.push(NodeInfo{host: "localhost", port: 15327});
    nodes.push(NodeInfo{host: "localhost", port: 15328});
    nodes.push(NodeInfo{host: "localhost", port: 15329});

    let mut hash_ring: HashRing<NodeInfo> = HashRing::new(nodes, 10);

    println!("{}", hash_ring.get_node(("hello").to_string()).unwrap());

    println!("{}", hash_ring.get_node(("dude").to_string()).unwrap());

    println!("{}", hash_ring.get_node(("martian").to_string()).unwrap());

    println!("{}", hash_ring.get_node(("tardis").to_string()).unwrap());

    hash_ring.remove_node(&NodeInfo{host: "localhost", port: 15329});

    println!("{}", hash_ring.get_node(("hello").to_string()).unwrap());

    hash_ring.add_node(&NodeInfo{host: "localhost", port: 15329});

    println!("{}", hash_ring.get_node(("hello").to_string()).unwrap());
}
```

### Contributing
 Just fork it, implement your changes and submit a pull request.

### License

MIT
