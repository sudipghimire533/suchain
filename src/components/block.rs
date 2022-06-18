use crate::components::current_timestamp;
use crate::components::hash::Hash;
use crate::components::Nonce;
use crate::components::BlockNumber;
use crate::chain::Chain;
use crate::components::transaction::Transaction;
use crate::components::transaction::TransactionCollection;
use crate::components::transaction::TransactionResult;
use crate::components::consensus::ProofOfWork;
use crate::components::consensus::Consensus;

use serde::Deserialize;
use serde::Serialize;

pub type BlockCollection = Vec<Block>;

#[derive(Clone, Eq, Serialize, Deserialize)]
pub struct BlockHeader {
    pub parent_block: Hash,
    pub nonce: Nonce,
    pub height: BlockNumber,
    pub timetamp: u64,
}

#[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
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
    pub fn new(chain: &Chain) -> Self {
        let parent_block = chain.get_latest_block();
        Block {
            header: BlockHeader {
                parent_block: parent_block.get_hash(),
                nonce: 0,
                height: parent_block.header.height + 1,
                timetamp: current_timestamp(),
            },
            transactions: vec![],
        }
    }

    pub fn create_and_add(chain: &mut Chain, transactions: Vec<Transaction>) -> TransactionResult {
        let mut new_block = Self::new(&chain);
        new_block.transactions = transactions;
        <ProofOfWork as Consensus>::prepare_block(chain, &mut new_block)
            .map_err(|e| format!("While preparing block: {e}"))?;

        chain.add_block(new_block)
            .map_err(|e| format!("While adding block: {e}").into())
    }

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
