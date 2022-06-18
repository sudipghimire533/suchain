pub mod block;
pub mod hash;
pub mod transaction;
pub mod origin;

pub type Nonce = u32;
pub type BlockNumber = u64;
pub type AccountId = hash::Hash;
pub type Balance = u64;
pub type SuHasher = sha3::Sha3_256;
pub const SU_HASHER_LEN: usize = 32_usize;
