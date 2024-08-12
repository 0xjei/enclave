use crate::{CiphernodeRegistered, ComputationRequested, EventConsumer, KeyshareCreated};

pub struct Aggregator {}
impl Aggregator {
    fn on_node_registered(&self, event: &CiphernodeRegistered) {}
    fn on_computation_requested(&self, event: &ComputationRequested) {}
    fn on_keyshare_created(&self, event: &KeyshareCreated) {}
}

impl EventConsumer for Aggregator {
    async fn consume(&self,event: &crate::EnclaveEvent) {
        
    }
}
