use ethers::{
    providers::Middleware,
    types::{Address, BlockId, Bytes, H256, U256, U64},
};
use tokio::runtime::Runtime;

#[derive(Debug)]
/// Blocking wrapper around an Ethers middleware, for use in synchronous contexts
/// (powered by a tokio runtime)
pub struct BlockingProvider<M> {
    provider: M,
    runtime: Runtime,
}

impl<M: Clone> Clone for BlockingProvider<M> {
    fn clone(&self) -> Self {
        Self { provider: self.provider.clone(), runtime: Runtime::new().unwrap() }
    }
}

impl<M: Middleware> BlockingProvider<M> {
    pub fn new(provider: M) -> Self {
        Self { provider, runtime: Runtime::new().unwrap() }
    }

    fn block_on<F: std::future::Future>(&self, f: F) -> F::Output {
        self.runtime.block_on(f)
    }

    pub fn get_block_number(&self) -> Result<U64, M::Error> {
        self.block_on(self.provider.get_block_number())
    }

    pub fn get_balance(&self, address: Address, block: Option<BlockId>) -> Result<U256, M::Error> {
        self.block_on(self.provider.get_balance(address, block))
    }

    pub fn get_transaction_count(
        &self,
        address: Address,
        block: Option<BlockId>,
    ) -> Result<U256, M::Error> {
        self.block_on(self.provider.get_transaction_count(address, block))
    }

    pub fn get_code(&self, address: Address, block: Option<BlockId>) -> Result<Bytes, M::Error> {
        self.block_on(self.provider.get_code(address, block))
    }

    pub fn get_storage_at(
        &self,
        address: Address,
        slot: H256,
        block: Option<BlockId>,
    ) -> Result<H256, M::Error> {
        self.block_on(self.provider.get_storage_at(address, slot, block))
    }
}