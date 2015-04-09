use crypto::md5::Md5;
use crypto::digest::Digest;
use std::collections::HashMap;
use std::collections::BinaryHeap;

pub struct HashRing<T> {
	replicas: isize,
	ring: HashMap<String, T>,
	sorted_keys: Vec<String>
}

impl<T: ToString+Clone> HashRing<T> {
	/// Creates a new hash ring with the specified nodes. Replicas is the number of virtual nodes each node has to make a better distribution.
	pub fn new(nodes: Vec<T>, replicas: isize) -> HashRing<T> {
		let mut new_hash_ring: HashRing<T> = HashRing {
			replicas: replicas,
			ring: HashMap::new(),
			sorted_keys: Vec::new()
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

		self.sorted_keys = BinaryHeap::from_vec(self.sorted_keys.clone()).into_sorted_vec();
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
			self.sorted_keys.remove(index);
		}
	}

	/// Gets the node a specific key belongs to
	pub fn get_node(&mut self, key: String) -> &T {
		let generated_key = self.gen_key(key);
		let nodes = self.sorted_keys.clone();

		for i in 0..nodes.len() {
			let node = &nodes[i];
			if generated_key <= *node {
				return self.ring.get(node).unwrap();
			}
		}

		let node = &nodes[0];
		return self.ring.get(node).unwrap();
	}

	/// Generates a key from a string value
	fn gen_key(&mut self, key: String) -> String {
		let mut md5: Md5 = Md5::new();
		md5.input_str(key.as_ref());
		return md5.result_str();
	}
}