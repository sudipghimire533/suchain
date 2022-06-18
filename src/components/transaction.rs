use std::borrow::Cow;

use crate::components::AccountId;
use crate::components::origin::Origin;
use crate::components::Balance;

use serde::Serialize;
use serde::Deserialize;

pub type TransactionCollection = Vec<Transaction>;
pub type TransactionResult = Result<(), Cow<'static, str>>;

#[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
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

#[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Transaction {
    pub operation: Operation,
    pub initiator: Origin,
}

impl core::fmt::Debug for Transaction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let tx_to_string = serde_json::to_string(&self)
            .map_err(|_| std::fmt::Error)?;
        write!(f, "{tx_to_string}")
    }
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
