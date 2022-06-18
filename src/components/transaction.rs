use serde::Serialize;

use crate::components::AccountId;
use crate::components::origin::Origin;
use crate::components::Balance;

pub type TransactionCollection = Vec<Transaction>;
pub type TransactionResult = Result<(), &'static str>;

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub enum Operation {
    Empty,
    Panic,
    DestroyAccount {
        account_id: AccountId,
    },
    TransferFund {
        sender: AccountId,
        receiver: AccountId,
        amount: Balance,
    },
    Airdrop {
        receiver: AccountId,
        amount: Balance,
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct Transaction {
    pub operation: Operation,
    pub initiator: Origin,
}

impl Operation {
    pub fn is_privilaged(&self, origin: &Origin) -> bool {
        match self {
            Operation::Empty | Operation::Panic =>
                true,
            Operation::DestroyAccount { account_id } =>
                origin.signed() == Some(account_id),
            Operation::TransferFund { sender, .. } =>
                origin.signed() == Some(sender),
            Operation::Airdrop { receiver, .. } =>
                origin.signed() == Some(receiver)
        }
    }
}
