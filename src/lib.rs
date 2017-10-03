#![crate_name = "hash_ring"]
#![crate_type = "lib"]

extern crate md5;
mod hash_ring;
pub use hash_ring::NodeInfo;
pub use hash_ring::HashRing;