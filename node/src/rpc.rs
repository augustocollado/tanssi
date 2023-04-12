//! A collection of node-specific RPC methods.
//! Substrate provides the `sc-rpc` crate, which defines the core RPC layer
//! used by Substrate nodes. This file extends those RPC definitions with
//! capabilities that are specific to this project's runtime configuration.

#![warn(missing_docs)]

use std::sync::Arc;

use test_runtime::{opaque::Block, AccountId, Index as Nonce};

use polkadot_primitives::Hash;
use sc_client_api::AuxStore;
use sc_consensus_manual_seal::rpc::ManualSeal;
use sc_consensus_manual_seal::rpc::ManualSealApiServer;
use sc_consensus_manual_seal::EngineCommand;
pub use sc_rpc::{DenyUnsafe, SubscriptionTaskExecutor};
use sc_transaction_pool_api::TransactionPool;
use sp_api::ProvideRuntimeApi;
use sp_block_builder::BlockBuilder;
use sp_blockchain::{Error as BlockChainError, HeaderBackend, HeaderMetadata};

/// A type representing all RPC extensions.
pub type RpcExtension = jsonrpsee::RpcModule<()>;

/// Full client dependencies
pub struct FullDeps<C, P> {
    /// The client instance to use.
    pub client: Arc<C>,
    /// Transaction pool instance.
    pub pool: Arc<P>,
    /// Whether to deny unsafe calls
    pub deny_unsafe: DenyUnsafe,
    /// Manual seal command sink
    pub command_sink: Option<futures::channel::mpsc::Sender<EngineCommand<Hash>>>,
}

/// Instantiate all RPC extensions.
pub fn create_full<C, P>(
    deps: FullDeps<C, P>,
) -> Result<RpcExtension, Box<dyn std::error::Error + Send + Sync>>
where
    C: ProvideRuntimeApi<Block>
        + HeaderBackend<Block>
        + AuxStore
        + HeaderMetadata<Block, Error = BlockChainError>
        + Send
        + Sync
        + 'static,
    C::Api: substrate_frame_rpc_system::AccountNonceApi<Block, AccountId, Nonce>,
    C::Api: BlockBuilder<Block>,
    P: TransactionPool + Sync + Send + 'static,
{
    use substrate_frame_rpc_system::{System, SystemApiServer};

    let mut module = RpcExtension::new(());
    let FullDeps {
        client,
        pool,
        deny_unsafe,
        command_sink,
    } = deps;

    module.merge(System::new(client, pool, deny_unsafe).into_rpc())?;

    if let Some(command_sink) = command_sink {
        module.merge(
            // We provide the rpc handler with the sending end of the channel to allow the rpc
            // send EngineCommands to the background block authorship task.
            ManualSeal::new(command_sink).into_rpc(),
        )?;
    };

    Ok(module)
}
