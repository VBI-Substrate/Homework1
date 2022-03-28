pub use self::gen_client::Client as TransactionPaymentClient;
// use codec::{Codec, Decode};
use jsonrpc_core::{Error as RpcError, ErrorCode, Result};
use jsonrpc_derive::rpc;
use sp_api::ProvideRuntimeApi;
use sp_blockchain::HeaderBackend;
use sp_runtime::{
	generic::BlockId,
	traits::{Block as BlockT},
};
use std::sync::Arc;
pub use pallet_kitties_rpc_runtime_api::KittiesApi as KittiesRuntimeApi;
use pallet_kitties::Kitty;

#[rpc]
pub trait KittiesApi<BlockHash> {
	#[rpc(name = "kitties_getKittyCnt")]
	fn get_kitty_cnt(&self, at: Option<BlockHash>) -> Result<u64>;
	fn get_kitty(&self, at: Option<BlockHash>) -> Result<Vec<Kitty>>;

}

/// A struct that implements the [`TransactionPaymentApi`].
pub struct KittiesStruct<C, P> {
	client: Arc<C>,
	_marker: std::marker::PhantomData<P>,
}

impl<C, P> KittiesStruct<C, P> {
	/// Create new `TransactionPayment` with the given reference to the client.
	pub fn new(client: Arc<C>) -> Self {
		Self { client, _marker: Default::default() }
	}
}

/// Error type of this RPC api.
pub enum Error {
	/// The transaction was not decodable.
	DecodeError,
	/// The call to runtime failed.
	RuntimeError,
}

impl From<Error> for i64 {
	fn from(e: Error) -> i64 {
		match e {
			Error::RuntimeError => 1,
			Error::DecodeError => 2,
		}
	}
}

impl<C, Block> KittiesApi<<Block as BlockT>::Hash>
	for KittiesStruct<C, Block>
where
	Block: BlockT,
	C: 'static + ProvideRuntimeApi<Block> + HeaderBackend<Block>,
    C::Api: KittiesRuntimeApi<Block>,
{
	fn get_kitty_cnt(
		&self,
		at: Option<<Block as BlockT>::Hash>,
	) -> Result<u64> {
		let api = self.client.runtime_api();
		let at = BlockId::hash(at.unwrap_or_else(||
			// If the block hash is not supplied assume the best block.
			self.client.info().best_hash));

        let result_api = api.get_kitty_cnt(&at);
		result_api.map_err(|e| RpcError {
			code: ErrorCode::ServerError(Error::RuntimeError.into()),
			message: "Unable to query dispatch info.".into(),
			data: Some(e.to_string().into()),
		})
	}
	fn get_kitty(
		&self,
		at: Option<<Block as BlockT>::Hash>,
	) -> Result<u64> {
		let api = self.client.runtime_api();
		let at = BlockId::hash(at.unwrap_or_else(||
			// If the block hash is not supplied assume the best block.
			self.client.info().best_hash));

        let result_api = api.get_kitty(&at);
		result_api.map_err(|e| RpcError {
			code: ErrorCode::ServerError(Error::RuntimeError.into()),
			message: "Unable to query dispatch info.".into(),
			data: Some(e.to_string().into()),
		})
	}

}