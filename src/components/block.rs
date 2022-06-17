use crate::components::hash::Hash;
use crate::components::Nonce;
use crate::components::BlockNumber;
use crate::components::transaction::TransactionCollection;
use serde::Serialize;

#[derive(Clone, Debug, Eq, Serialize)]
pub struct BlockHeader {
    pub parent_block: Hash,
    pub nonce: Nonce,
    pub height: BlockNumber,
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
    pub fn get_hash(&self) -> Hash {
        let _block_as_json = serde_json::to_string(self)
            .expect("Cannot represent Block{} as json string");
        todo!()
    }
}
