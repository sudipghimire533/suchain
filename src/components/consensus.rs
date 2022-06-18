use crate::chain::Chain;
use crate::components::block::Block;
use crate::components::transaction::TransactionResult;

pub trait Consensus {
    fn add_block(chain: &mut Chain, new_block: Block) -> TransactionResult;
}

pub struct ProffOfWork;

impl Consensus for ProffOfWork {
    fn add_block(chain: &mut Chain, new_block: Block) -> TransactionResult {
        todo!()
    }
}
