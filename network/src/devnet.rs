use async_trait::async_trait;
use tokio::sync::mpsc;

use crate::{AuthorizedNetwork, BootstrapPoint};
use simperby_common::crypto::*;

/// An instance of `simperby::network::AuthorizedNetwork`
pub struct DevNet {}

#[async_trait]
impl AuthorizedNetwork for DevNet {
    /// Joins the network with an authorized identity.
    async fn new(
        _public_key: PublicKey,
        _private_key: PrivateKey,
        _bootstrap_points: Vec<BootstrapPoint>,
        _network_id: String,
    ) -> Result<Self, String>
    where
        Self: Sized,
    {
        unimplemented!("not implemented");
    }
    /// Broadcasts a message to the network, after signed by the key given to this instance.
    async fn broadcast(&self, _message: &[u8]) -> Result<(), String> {
        unimplemented!("not implemented");
    }
    /// Creates a receiver for every message broadcasted to the network, except the one sent by this instance.
    async fn create_recv_queue(&self) -> Result<mpsc::Receiver<Vec<u8>>, ()> {
        unimplemented!("not implemented");
    }
    /// Provides the estimated list of live nodes that are eligible and identified by their public keys.
    async fn get_live_list(&self) -> Result<Vec<PublicKey>, ()> {
        unimplemented!("not implemented");
    }
}

#[cfg(test)]
mod test {
    #[test]
    #[ignore]
    /// Test if this node receives a message from another node.
    fn broadcast_once() {
        unimplemented!("not implemented");
    }

    #[test]
    #[ignore]
    /// Test if this node receives multiple messages from another node.
    fn broadcast_multiple_times() {
        unimplemented!("not implemented");
    }

    #[test]
    #[ignore]
    /// Test if this node receives multiple messages from other nodes.
    fn broadcast_from_multiple_nodes() {
        unimplemented!("not implemented");
    }

    #[test]
    #[ignore]
    /// Test if this node receives multiple messages from other nodes,
    /// ... when several nodes are joining and leaving the network.
    fn broadcast_from_multiple_nodes_with_flexible_network() {
        unimplemented!("not implemented");
    }

    #[test]
    #[ignore]
    /// Test if this node retrieves the list of all nodes in the network.
    fn get_live_list_once() {
        unimplemented!("not implemented");
    }

    #[test]
    #[ignore]
    /// Test if this node retrieves lists of all nodes in the network multiple times.
    fn get_live_list_multiple_times() {
        unimplemented!("not implemented");
    }

    #[test]
    #[ignore]
    /// Test if this node retrieves the list of all nodes in the network,
    /// ... when several nodes are joining and leaving the network.
    fn get_live_list_with_flexible_network() {
        unimplemented!("not implemented");
    }
}