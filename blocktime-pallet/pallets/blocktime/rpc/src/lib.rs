//! RPC interface for the transaction payment module.

use jsonrpc_core::{Error as RpcError, ErrorCode, Result};
use jsonrpc_derive::rpc;
use sp_api::ProvideRuntimeApi;
use sp_blockchain::HeaderBackend;
use sp_runtime::{generic::BlockId, traits::Block as BlockT};
use std::sync::Arc;
use pallet_blocktime_runtime_api::BlocktimeApi as BlocktimeRuntimeApi;

#[rpc]
pub trait BlocktimeApi<BlockHash> {
	#[rpc(name = "pellet_blocktime_getCurrentBlockTime")]
	fn get_current_block_time(&self, at: Option<BlockHash>) -> Result<u32>;
}
pub struct Blocktime<C, M> {
	client: Arc<C>,
	_marker: std::marker::PhantomData<M>,
}
impl<V, D> Blocktime<V, D> {
	pub fn new(client: Arc<V>) -> Self {
		Self {
			client,
			_marker: Default::default(),
		}
	}
}

impl<C, Block> BlocktimeApi<<Block as BlockT>::Hash> for Blocktime<C, Block>
where
	Block: BlockT,
	C: Send + Sync + 'static,
	C: ProvideRuntimeApi<Block>,
	C: HeaderBackend<Block>,
	C::Api: BlocktimeRuntimeApi<Block>,
{
	fn get_current_block_time(&self, at: Option<<Block as BlockT>::Hash>) -> Result<u32> {
		let api = self.client.runtime_api();
		let at = BlockId::hash(at.unwrap_or_else(|| self.client.info().best_hash));
		let runtime_api_result = api.get_current_block_time(&at);
	}
}
