use crate::helpers::{RpcWSClient, SignerProvider, WithChainId};
use actix::prelude::*;
use actix::Addr;
use alloy::{primitives::Address, sol};
use alloy::{
    primitives::{Bytes, U256},
    rpc::types::TransactionReceipt,
};
use anyhow::Result;
use events::Shutdown;
use events::{BusError, E3id, EnclaveErrorType, PlaintextAggregated, Subscribe};
use events::{EnclaveEvent, EventBus};
use tracing::info;

sol!(
    #[sol(rpc)]
    IEnclave,
    "../../evm/artifacts/contracts/interfaces/IEnclave.sol/IEnclave.json"
);

/// Consumes events from the event bus and calls EVM methods on the Enclave.sol contract
pub struct EnclaveSolWriter {
    provider: WithChainId<SignerProvider<RpcWSClient>, RpcWSClient>,
    contract_address: Address,
    bus: Addr<EventBus<EnclaveEvent>>,
}

impl EnclaveSolWriter {
    pub fn new(
        bus: &Addr<EventBus<EnclaveEvent>>,
        provider: &WithChainId<SignerProvider<RpcWSClient>, RpcWSClient>,
        contract_address: Address,
    ) -> Result<Self> {
        Ok(Self {
            provider: provider.clone(),
            contract_address,
            bus: bus.clone(),
        })
    }

    pub async fn attach(
        bus: &Addr<EventBus<EnclaveEvent>>,
        provider: &WithChainId<SignerProvider<RpcWSClient>, RpcWSClient>,
        contract_address: &str,
    ) -> Result<Addr<EnclaveSolWriter>> {
        let addr = EnclaveSolWriter::new(bus, provider, contract_address.parse()?)?.start();
        bus.send(Subscribe::new("PlaintextAggregated", addr.clone().into()))
            .await?;

        bus.send(Subscribe::new("Shutdown", addr.clone().into()))
            .await?;

        Ok(addr)
    }
}

impl Actor for EnclaveSolWriter {
    type Context = actix::Context<Self>;
}

impl Handler<EnclaveEvent> for EnclaveSolWriter {
    type Result = ();
    fn handle(&mut self, msg: EnclaveEvent, ctx: &mut Self::Context) -> Self::Result {
        match msg {
            EnclaveEvent::PlaintextAggregated { data, .. } => {
                // Only publish if the src and destination chains match
                if self.provider.get_chain_id() == data.src_chain_id {
                    ctx.notify(data);
                }
            }
            EnclaveEvent::Shutdown { data, .. } => ctx.notify(data),
            _ => (),
        }
    }
}

impl Handler<PlaintextAggregated> for EnclaveSolWriter {
    type Result = ResponseFuture<()>;
    fn handle(&mut self, msg: PlaintextAggregated, _: &mut Self::Context) -> Self::Result {
        Box::pin({
            let e3_id = msg.e3_id.clone();
            let decrypted_output = msg.decrypted_output.clone();
            let contract_address = self.contract_address.clone();
            let provider = self.provider.clone();
            let bus = self.bus.clone();

            async move {
                let result =
                    publish_plaintext_output(provider, contract_address, e3_id, decrypted_output)
                        .await;
                match result {
                    Ok(receipt) => {
                        info!(tx=%receipt.transaction_hash, "tx")
                    }
                    Err(err) => bus.err(EnclaveErrorType::Evm, err),
                }
            }
        })
    }
}

impl Handler<Shutdown> for EnclaveSolWriter {
    type Result = ();
    fn handle(&mut self, _: Shutdown, ctx: &mut Self::Context) -> Self::Result {
        ctx.stop();
    }
}

async fn publish_plaintext_output(
    provider: WithChainId<SignerProvider<RpcWSClient>, RpcWSClient>,
    contract_address: Address,
    e3_id: E3id,
    decrypted_output: Vec<u8>,
) -> Result<TransactionReceipt> {
    let e3_id: U256 = e3_id.try_into()?;
    let decrypted_output = Bytes::from(decrypted_output);
    let proof = Bytes::from(vec![1]);
    let contract = IEnclave::new(contract_address, provider.get_provider());
    let builder = contract.publishPlaintextOutput(e3_id, decrypted_output, proof);
    let receipt = builder.send().await?.get_receipt().await?;
    Ok(receipt)
}
