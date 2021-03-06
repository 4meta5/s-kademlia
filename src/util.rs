//! utilities
use crate::node_id::NodeId;
use disco::DiscoHash;

// for testing purposes only (TODO: remove in lieu of `quickcheck`)
pub trait Random<T> {
    fn random(len: usize) -> T;
}

impl Random<NodeId> for NodeId {
    // Note: does not generate keypair, just generates a random byte array
    fn random(len: usize) -> NodeId {
        NodeId {
            discohash: DiscoHash::random(len),
        }
    }
}

#[cfg(test)]
pub mod test {
    use crate::node::{NodeInfo, NodeStatus};
    use crate::node_id::NodeId;
    use crate::store::{NodeBucket, NodeTable};
    use crate::ed25519::Keypair;
    use std::collections::VecDeque;
    use std::net::{IpAddr, Ipv4Addr, SocketAddr};
    use rand;

    pub static ADDR: &'static str = "127.0.0.1:8008";

    // TODO: eventually add `net::SocketAddr` and `NodeStatus` as input parameters
    pub fn new_node_info(id: NodeId) -> NodeInfo {
        let socket = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 8080);
        let status = NodeStatus::Connected;
        let new_node = NodeInfo { id, socket, status };
        new_node
    }

    #[test]
    fn new_node_info_succeeds() {
        let key = Keypair::generate(&mut rand::thread_rng());
        let node_id = NodeId::from_public_key(key.public);
        let new_node = new_node_info(node_id);
        assert_eq!(new_node.port(), 8080);
        assert_eq!(new_node.ip(), IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)));
    }

    fn new_node_bucket(node_count: usize) -> NodeBucket {
        let mut bucket = NodeBucket {
            nodes: VecDeque::new(),
            node_count,
        };
        // should prevent duplicate NodeId generation eventually
        for i in 0..node_count {
            let key = Keypair::generate(&mut rand::thread_rng());
            let node_id = NodeId::from_public_key(key.public);
            let new_node = new_node_info(node_id);
            bucket.nodes.push_back(new_node);
        }
        bucket
    }

    #[test]
    fn new_node_bucket_succeeds() {
        let new_bucket = new_node_bucket(10);
        assert_eq!(new_bucket.nodes.len(), 10);
    }
}
