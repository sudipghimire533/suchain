use std::collections::HashMap;
use std::time::Duration;
use std::borrow::Cow;

use crate::components::AccountId;
use crate::components::Balance;
use crate::components::block::Block;
use crate::components::block::BlockCollection;
use crate::components::hash::Hash;
use crate::components::consensus::Consensus;
use crate::components::consensus::ProofOfWork;
use crate::components::transaction::Operation;
use crate::components::transaction::Transaction;
use crate::components::transaction::TransactionResult;

#[derive(Clone, Debug)]
pub struct ChainProperties {
    pub exestinsial_deposit: Balance,
    pub difficulty: usize,
    pub time_tolorant: Duration,
    pub consensus: ProofOfWork,
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

#[derive(Clone, Debug)]
pub struct Chain {
    pub chain_info: Cow<'static, str>,
    pub blocks: BlockCollection,
    pub accounts: MappedAccountInfo,
    pub properties: ChainProperties,
    system_account: AccountId,
}

impl Chain {
    pub fn new(chain_info: Cow<'static, str>,
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
            Operation::Panic => self.panic_operation(),
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
    pub fn get_latest_block(&self) -> &Block {
        self.blocks
            .last()
            .expect("There will always be at least one bloock in chain")
    }

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
                match err.as_ref() {
                    "sender balance too low"
                        | "can't kill sender" => "system allowence too low".into(),
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

    pub fn panic_operation(&mut self) -> TransactionResult {
        panic!("A panic becaused operation demands..")
    }

    pub fn add_block(&mut self, new_block: Block) -> TransactionResult {
        <ProofOfWork as Consensus>::add_new_block(self, new_block)
    }
}
