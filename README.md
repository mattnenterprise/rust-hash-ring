rust-hash-ring
================

Consistent Hashing library for Rust

[![Build Status](https://travis-ci.org/mattnenterprise/rust-hash-ring.svg)](https://travis-ci.org/mattnenterprise/rust-hash-ring)
[![crates.io](http://meritbadge.herokuapp.com/hash_ring)](https://crates.io/crates/hash_ring)
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

    println!("{}", hash_ring.get_node(String::from_str("hello")).to_string());

    println!("{}", hash_ring.get_node(String::from_str("dude")).to_string());

    println!("{}", hash_ring.get_node(String::from_str("martian")).to_string());

    println!("{}", hash_ring.get_node(String::from_str("tardis")).to_string());

    hash_ring.remove_node(&NodeInfo{host: "localhost", port: 15329});

    println!("{}", hash_ring.get_node(String::from_str("hello")).to_string());

    hash_ring.add_node(&NodeInfo{host: "localhost", port: 15329});

    println!("{}", hash_ring.get_node(String::from_str("hello")).to_string());
}
```

### Contributing
 Just fork it, implement your changes and submit a pull request.

### License

MIT
