pub mod block;
pub mod hash;
pub mod transaction;
pub mod origin;

pub type Nonce = u32;
pub type BlockNumber = u64;
pub type AccountId = hash::Hash;
pub type Balance = u64;
