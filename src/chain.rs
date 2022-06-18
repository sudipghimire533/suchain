use std::collections::HashMap;

use crate::components::AccountId;
use crate::components::Balance;
use crate::components::block::Block;
use crate::components::hash::Hash;
use crate::components::transaction::Operation;
use crate::components::transaction::Transaction;
use crate::components::transaction::TransactionResult;

pub struct ChainProperties {
    pub exestinsial_deposit: Balance,
}

pub type MappedAccountInfo = HashMap<AccountId, AccountInfo>;

#[derive(PartialEq, Eq, Debug, Clone)]
pub struct AccountInfo {
    balance: Balance,
}

impl Default for AccountInfo {
    fn default() -> Self {
        AccountInfo {
            balance: 0u8.into(),
        }
    }
}

pub struct Chain {
    pub chain_info: &'static str,
    pub blocks: Vec<Block>,
    pub accounts: MappedAccountInfo,
    pub properties: ChainProperties,
    system_account: AccountId,
}

impl Chain {
    pub fn new(chain_info: &'static str,
               properties: ChainProperties,
               system_allowance: Balance,
               ) -> Self
    {
        let system_account = Hash::raw([u8::MAX; crate::components::SU_HASHER_LEN]);
        let system_account_info = AccountInfo {
            balance: system_allowance,
        };

        let predefined_accounts = vec![(system_account.clone(), system_account_info)];
        Chain {
            chain_info,
            blocks: vec![Block::get_genesis()],
            accounts: predefined_accounts.into_iter().collect(),
            properties,
            system_account,
        }
    }

    pub fn execute(&mut self, transaction: Transaction) -> TransactionResult {
        if !transaction.operation.is_privilaged(&transaction.initiator) {
            Err("Insufficient permission")?;
        }

        match transaction.operation {
            Operation::Empty => self.empty_operation(),
            Operation::DestroyAccount { account_id } =>
                self.destroy_account(account_id),
            Operation::TransferFund { sender, receiver, amount } =>
                self.transfer_fund(sender, receiver, amount, true),
            Operation::Airdrop { receiver, amount } =>
                self.airdrop(receiver, amount),
        }
    }
}

impl Chain {
    pub fn transfer_fund<'chain>(
        &'chain mut self,
        sender: AccountId,
        receiver: AccountId,
        amount: Balance,
        keep_alive: bool,
    ) -> TransactionResult {
        let sender_balance = self.accounts.get(&sender)
            .ok_or("sender account doesn't exists")?
            .balance;

        if sender_balance < amount {
            Err("sender balance too low")?;
        }

        let sender_usable_balance = sender_balance - self.properties.exestinsial_deposit;
        if sender_usable_balance < amount && keep_alive {
            Err("can't kill sender")?;
        }

        let receiver_info = self.accounts.entry(receiver).or_default();
        let receiver_balance = receiver_info.balance;

        if receiver_balance + amount < self.properties.exestinsial_deposit {
            Err("amount too low to create receiver account")?;
        }

        (*receiver_info).balance
            .checked_add(amount)
            .ok_or("receiver balance reached sky")?;

        self.accounts.entry(sender).and_modify(|sender_info|{
            (*sender_info).balance -= amount
        });

        Ok(())
    }

    pub fn airdrop(&mut self, receiver: AccountId, amount: Balance) -> TransactionResult {
        let system_account = self.system_account.clone();
        self.transfer_fund(system_account, receiver, amount, true)
            .map_err(|err| {
                match err {
                    "sender balance too low"
                        | "can't kill sender" => "system allowence too low",
                    _ => err
                }
            })
    }

    fn destroy_account(&mut self, account: AccountId) -> TransactionResult {
        let system_account = self.system_account.clone();
        let account_balance = self.accounts
            .get(&account)
            .ok_or("account does not exists to destroy")?
            .balance;

        self.transfer_fund(account, system_account, account_balance, false)
    }

    pub fn empty_operation(&mut self) -> TransactionResult {
        Ok(())
    }
}
