use crate::components::current_timestamp;
use crate::components::hash::Hash;
use crate::components::Nonce;
use crate::components::BlockNumber;
use crate::components::transaction::TransactionCollection;

use serde::Serialize;

pub type BlockCollection = Vec<Block>;

#[derive(Clone, Debug, Eq, Serialize)]
pub struct BlockHeader {
    pub parent_block: Hash,
    pub nonce: Nonce,
    pub height: BlockNumber,
    pub timetamp: u64,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub struct Block {
    pub header: BlockHeader,
    pub transactions: TransactionCollection,
}

impl PartialEq for BlockHeader {
    fn eq(&self, other: &Self) -> bool {
        ( self.parent_block == other.parent_block )
            && (self.height == other.height)
            && (self.nonce == other.nonce )
    }
}

impl Block {
    pub fn get_genesis() -> Self {
        let block_height: BlockNumber = 1u32.into();
        let parent_hash: Hash = Hash::raw([0u8; 32]);
        let nonce: Nonce = 0u32.into();
        let transactions = vec![];

        Block {
            header: BlockHeader {
                parent_block: parent_hash,
                nonce,
                height: block_height,
                timetamp: current_timestamp(),
            },
            transactions,
        }
    }

    pub fn get_hash(&self) -> Hash {
        let block_as_json = serde_json::to_string(self)
            .expect("Cannot represent Block{} as json string");
        
        Hash::new(block_as_json.as_bytes())
    }
}
