use crate::BlockIdProvider;
use reth_interfaces::Result;
use reth_primitives::{rpc::BlockId, BlockNumber, TransactionSigned, TxHash, TxNumber};
use std::ops::RangeBounds;

///  Client trait for fetching [TransactionSigned] related data.
#[auto_impl::auto_impl(&, Arc)]
pub trait TransactionsProvider: BlockIdProvider + Send + Sync {
    /// Get transaction by id.
    fn transaction_by_id(&self, id: TxNumber) -> Result<Option<TransactionSigned>>;

    /// Get transaction by transaction hash.
    fn transaction_by_hash(&self, hash: TxHash) -> Result<Option<TransactionSigned>>;

    /// Get transactions by block id.
    fn transactions_by_block(&self, block: BlockId) -> Result<Option<Vec<TransactionSigned>>>;

    /// Get transactions by block range.
    fn transactions_by_block_range(
        &self,
        range: impl RangeBounds<BlockNumber>,
    ) -> Result<Vec<Vec<TransactionSigned>>>;
}
