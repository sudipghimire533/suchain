use serde::Deserialize;
use serde::Serialize;

use crate::chain::{Chain, ChainProperties};
use crate::components::block::Block;
use crate::components::Nonce;
use crate::components::transaction::TransactionResult;

use super::current_timestamp;

pub trait Consensus {
    fn prepare_block(chain: &mut Chain, new_block: &mut Block) -> TransactionResult;
    fn verify_new_block(last_block: &Block, new_block: &Block, properties: &ChainProperties) -> TransactionResult;
    fn add_new_block(chain: &mut Chain, new_block: Block) -> TransactionResult;
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ProofOfWork;

impl Consensus for ProofOfWork {
    fn prepare_block(chain: &mut Chain, new_block: &mut Block) -> TransactionResult {
        let difficulty = chain.properties.difficulty;
        let latest_block = chain.get_latest_block();

        let latest_block_hash = latest_block.get_hash();
        new_block.header.parent_block = latest_block_hash;

        let lower_time_bound = new_block.header.timestamp;
        let upper_time_vound = lower_time_bound
            .checked_add(chain.properties.time_tolorant.as_secs())
            .expect("oops! is this of time? I though time was eternal :(");
        let time_range = lower_time_bound..upper_time_vound;

        for produced_time in time_range{
            new_block.header.timestamp = produced_time;
            let nonce_range = Nonce::MIN..Nonce::MAX;

            for nonce in nonce_range.clone() {
                if new_block
                    .get_hash()
                    .difficulty_verified(difficulty)
                {
                    return Ok(());
                }
                new_block.header.nonce = nonce;
            }
        }

        Self::verify_new_block(latest_block, new_block, &chain.properties)
            .map_err(|e| format!("Post verification error: {e}"))?;

        Err("Cannot pass difficulty with any value".into())
    }

    fn add_new_block(chain: &mut Chain, new_block: Block) -> TransactionResult {
        let parent_block = chain.get_latest_block();
        
        Self::verify_new_block(&parent_block, &new_block, &chain.properties)
            .map_err(|e| format!("Verifying new block: {e}"))?;

        // A simple rollback mechanism
        let old_accounts_state = chain.accounts.clone();

        for (tx_index, transaction) in new_block.transactions.iter().enumerate() {
            let transaction_result = chain.execute(transaction.clone());

            if let Err(tx_err) = transaction_result {
                chain.accounts = old_accounts_state;
                return Err(
                    format!(
                        "Error while performing {tx_index}th transaction.\
                        Transaction: {transaction:?}.\
                        Error: {tx_err}"
                    ).into()
                );
            }
        }

        chain.blocks.push(new_block);

        Ok(())
    }

    fn verify_new_block(last_block: &Block, new_block: &Block, properties: &ChainProperties) -> TransactionResult {
        // Verify the parent hash
        let written_parent_hash = &new_block.header.parent_block;
        let expected_parent_hash = last_block.get_hash();
        if written_parent_hash != &expected_parent_hash {
            Err("mismatched parent hash in new block header")?;
        }

        // Verify the block age
        let block_timestamp = new_block.header.timestamp;
        let timestamp_difference = current_timestamp() - block_timestamp;
        let maximum_acceptable_difference = properties.time_tolorant.as_secs();
        if timestamp_difference > maximum_acceptable_difference {
            Err("block have aged too much")?;
        }

        // Verify the work done
        let difficulty = properties.difficulty;
        let new_block_hash = new_block.get_hash();
        if !new_block_hash.difficulty_verified(difficulty) {
            Err("new block hash do not pass difficulty")?;
        }

        Ok(())
    }
}
