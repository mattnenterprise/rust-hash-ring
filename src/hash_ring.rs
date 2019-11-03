use fasthash::{XXHasher};
use std::hash::{Hash, Hasher};
use std::collections::HashMap;
use std::collections::BinaryHeap;

/// As a convenience, rust-hash-ring provides a default struct to hold node
/// information. It is optional and you can define your own.
#[derive(Clone, Debug, PartialEq)]
pub struct NodeInfo {
    pub host: &'static str,
    pub port: u16,
}

impl ToString for NodeInfo {
    fn to_string(&self) -> String {
        format!("{}:{}", self.host, self.port)
    }
}

/// HashRing
#[derive(Debug, Clone)]
pub struct HashRing<T> {
    replicas: isize,
    ring: HashMap<String, T>,
    sorted_keys: Vec<String>,
}

impl<T: ToString + Clone> HashRing<T> {
    /// Creates a new hash ring with the specified nodes.
    /// Replicas is the number of virtual nodes each node has to make a better distribution.
    pub fn new(nodes: Vec<T>, replicas: isize) -> HashRing<T> {
        let mut new_hash_ring: HashRing<T> = HashRing {
            replicas: replicas,
            ring: HashMap::new(),
            sorted_keys: Vec::new(),
        };

        for i in 0..nodes.len() {
            let n = &nodes[i];
            new_hash_ring.add_node(n);
        }
        return new_hash_ring;
    }

    /// Adds a node to the hash ring
    pub fn add_node(&mut self, node: &T) {
        for i in 0..self.replicas {
            let key = self.gen_key(format!("{}:{}", node.to_string(), i));
            self.ring.insert(key.clone(), (*node).clone());
            self.sorted_keys.push(key.clone());
        }

        self.sorted_keys = BinaryHeap::from(self.sorted_keys.clone()).into_sorted_vec();
    }

    /// Deletes a node from the hash ring
    pub fn remove_node(&mut self, node: &T) {
        for i in 0..self.replicas {
            let key = self.gen_key(format!("{}:{}", node.to_string(), i));
            self.ring.remove(&key);
            let mut index = 0;
            for j in 0..self.sorted_keys.len() {
                if self.sorted_keys[j] == key {
                    index = j;
                    break;
                }
            }
            if self.sorted_keys.len() > index{
                self.sorted_keys.remove(index);
            }
        }
    }

    /// Gets the node a specific key belongs to
    pub fn get_node(&mut self, key: String) -> Option<&T> {
        if self.sorted_keys.is_empty() {
            return None;
        }

        let generated_key = self.gen_key(key);
        let nodes = self.sorted_keys.clone();

        for i in 0..nodes.len() {
            let node = &nodes[i];
            if generated_key <= *node {
                return Some(self.ring.get(node).unwrap());
            }
        }

        let node = &nodes[0];
        return Some(self.ring.get(node).unwrap());
    }

    /// Generates a key from a string value
    fn gen_key(&mut self, key: String) -> String {
        let digest = hash(&key);
        format!("{:x}", digest)
    }
}

fn hash<H: Hash>(t: &H) -> u64 {
    let mut s: XXHasher = Default::default();
    t.hash(&mut s);
    s.finish()
}

#[cfg(test)]
mod test {
    use hash_ring::{NodeInfo, HashRing};

    // Defines a NodeInfo for a localhost address with a given port.
    fn node(port: u16) -> NodeInfo {
        NodeInfo {
            host: "localhost",
            port,
        }
    }

    #[test]
    fn test_empty_ring() {
        let mut hash_ring: HashRing<NodeInfo> = HashRing::new(vec![], 10);
        assert_eq!(None, hash_ring.get_node("hello".to_string()));
    }

    #[test]
    fn test_default_nodes() {
        let mut nodes: Vec<NodeInfo> = Vec::new();
        nodes.push(node(15324));
        nodes.push(node(15325));
        nodes.push(node(15326));
        nodes.push(node(15327));
        nodes.push(node(15328));
        nodes.push(node(15329));


        let mut hash_ring: HashRing<NodeInfo> = HashRing::new(nodes, 10);

        assert_eq!(Some(&node(15325)), hash_ring.get_node("hello".to_string()));
        assert_eq!(Some(&node(15329)), hash_ring.get_node("dude".to_string()));

        hash_ring.remove_node(&node(15329));
        assert_eq!(Some(&node(15325)), hash_ring.get_node("hello".to_string()));

        hash_ring.add_node(&node(15329));
        assert_eq!(Some(&node(15329)), hash_ring.get_node("dude".to_string()));
    }

    #[derive(Clone)]
    struct CustomNodeInfo {
        pub host: &'static str,
        pub port: u16,
    }

    impl ToString for CustomNodeInfo {
        fn to_string(&self) -> String {
            format!("{}:{}", self.host, self.port)
        }
    }


    #[test]
    fn test_custom_nodes() {

        let mut nodes: Vec<CustomNodeInfo> = Vec::new();
        nodes.push(CustomNodeInfo {
            host: "localhost",
            port: 15324,
        });
        nodes.push(CustomNodeInfo {
            host: "localhost",
            port: 15325,
        });
        nodes.push(CustomNodeInfo {
            host: "localhost",
            port: 15326,
        });
        nodes.push(CustomNodeInfo {
            host: "localhost",
            port: 15327,
        });
        nodes.push(CustomNodeInfo {
            host: "localhost",
            port: 15328,
        });
        nodes.push(CustomNodeInfo {
            host: "localhost",
            port: 15329,
        });

        let mut hash_ring: HashRing<CustomNodeInfo> = HashRing::new(nodes, 10);

        assert_eq!(
            Some("localhost:15325".to_string()),
            hash_ring.get_node("hello".to_string()).map(
                |x| x.to_string(),
            )
        );
        assert_eq!(
            Some("localhost:15329".to_string()),
            hash_ring.get_node("dude".to_string()).map(
                |x| x.to_string(),
            )
        );

        hash_ring.remove_node(&CustomNodeInfo {
            host: "localhost",
            port: 15329,
        });
        assert_eq!(
            Some("localhost:15325".to_string()),
            hash_ring.get_node("hello".to_string()).map(
                |x| x.to_string(),
            )
        );

        hash_ring.add_node(&CustomNodeInfo {
            host: "localhost",
            port: 15329,
        });
        assert_eq!(
            Some("localhost:15325".to_string()),
            hash_ring.get_node("hello".to_string()).map(
                |x| x.to_string(),
            )
        );

    }
}
