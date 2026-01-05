pub mod account;
pub mod block;
pub mod commitment;
pub mod encoding;
pub mod pubkey;
pub mod transaction;

pub use account::{Account, AccountInfo};
pub use block::{Block, BlockEncoding};
pub use commitment::Commitment;
pub use encoding::Encoding;
pub use pubkey::Pubkey;
pub use transaction::{Transaction, TransactionEncoding};

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct RpcContext {
    pub slot: u64,
}


