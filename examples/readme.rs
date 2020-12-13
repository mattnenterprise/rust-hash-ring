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

    println!("Key: '{}', Node: {}", "hello" , hash_ring.get_node(("hello").to_string()).unwrap());

    println!("Key: '{}', Node: {}", "dude", hash_ring.get_node(("dude").to_string()).unwrap());

    println!("Key: '{}', Node: {}", "martian", hash_ring.get_node(("martian").to_string()).unwrap());

    println!("Key: '{}', Node: {}", "tardis", hash_ring.get_node(("tardis").to_string()).unwrap());

    hash_ring.remove_node(&NodeInfo{host: "localhost", port: 15329});

    println!("Key: '{}', Node: {}", "hello", hash_ring.get_node(("hello").to_string()).unwrap());

    hash_ring.add_node(&NodeInfo{host: "localhost", port: 15329});

    println!("Key: '{}', Node: {}", "hello", hash_ring.get_node(("hello").to_string()).unwrap());
}
