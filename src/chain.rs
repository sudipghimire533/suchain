use crate::components::AccountId;
use crate::components::Balance;
use crate::components::block::Block;
use crate::components::transaction::Operation;
use crate::components::transaction::Transaction;
use crate::components::transaction::TransactionResult;

pub struct Chain {
    pub chain_info: &'static str,
    pub blocks: Vec<Block>,
}

impl Chain {
    pub fn execute(&mut self, transaction: Transaction) -> TransactionResult {
        if !transaction.operation.is_privilaged(&transaction.initiator) {
            Err("Insufficient permission")?;
        }

        match transaction.operation {
            Operation::Empty => self.empty_operation(),
            Operation::DestroyAccount { account_id } =>
                self.destroy_account(account_id),
            Operation::TransferFund { sender, receiver, amount } =>
                self.transfer_fund(sender, receiver, amount),
        }
    }
}

pub trait ChainOperation {
    fn empty_operation(&mut self) -> TransactionResult {
        Ok(())
    }
    fn transfer_fund(&mut self, sender: AccountId, receiver: AccountId, amount: Balance) -> TransactionResult;
    fn destroy_account(&mut self, account: AccountId) -> TransactionResult;
}

impl ChainOperation for Chain {
    fn transfer_fund(&mut self, _sender: AccountId, _receiver: AccountId, _amount: Balance) -> TransactionResult {
        todo!()
    }

    fn destroy_account(&mut self, _account: AccountId) -> TransactionResult {
        todo!()
    }
}
