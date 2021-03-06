use crate::node_id::{KadMetric, NodeId};
use crate::store::{NodeBucket, NodeTable};
use std::{
    cmp::PartialEq,
    net::{IpAddr, SocketAddr},
    sync::{Arc, Mutex},
};

#[derive(Clone, Debug)]
pub struct NodeInfo {
    /// Identifier
    pub id: NodeId,
    /// IP Address, Port
    pub socket: SocketAddr,
    /// Status of the node
    pub status: NodeStatus,
}

impl PartialEq for NodeInfo {
    fn eq(&self, other: &NodeInfo) -> bool {
        self.id == other.id && self.socket == other.socket && self.status == other.status
    }
}

impl NodeInfo {
    /// Get node's IpAddr
    pub fn ip(&self) -> IpAddr {
        self.socket.ip()
    }

    /// Get node's port
    pub fn port(&self) -> u16 {
        self.socket.port()
    }
}

/// Status
///
/// reveals connected or not connected status
#[derive(PartialEq, Eq, Debug, Copy, Clone)]
pub enum NodeStatus {
    /// Node is connected.
    Connected,
    /// Node is not connected
    Disconnected,
}

#[cfg(test)]
mod tests {
    use super::{NodeInfo, NodeStatus};
    use crate::node_id::NodeId;
    use std::net::{IpAddr, Ipv4Addr, SocketAddr};
    // import test scaffolding
    use crate::util::test::*;

    // TODO
}
