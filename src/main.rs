use std::time::Duration;

pub mod components;
pub mod chain;

use chain::ChainProperties;
use components::consensus::ProofOfWork;
use components::Balance;
use components::AccountId;
use components::transaction::Transaction;
use components::origin::Origin;
use components::transaction::Operation;
use components::block::Block;

fn main() -> Result<(), i32> {
    //= Initialize suchain node
    print!("Creating fresh suchain node...");
    const SU_CHAIN_INFO: &'static str = "SuChain testnet v0.1";
    const SU_CHAIN_SUPPLY: Balance = 10_00_000;
    let suchain_prop = ChainProperties {
        consensus: ProofOfWork,
        difficulty: 2,
        exestinsial_deposit: 50,
        // For testing purpose let's keep time_tolorant to only 10 second.
        // But in production chain it ususally is in range of an hour or two
        time_tolorant: Duration::from_secs(30),
    };
    let mut suchain = chain::Chain::new(SU_CHAIN_INFO, suchain_prop, SU_CHAIN_SUPPLY);
    println!("..Done");
    println!("Node Info: {suchain:?}\n\n\n");
    //= Done initialize suchain node

    // Create a account and ask for airdrop
    let alice = AccountId::new("Suchain Alice");
    let alice_airdrop_amount = 100;
    print!("Creating alice ({alice:?}) and airdropping {alice_airdrop_amount} unit..");
    let tx_airdrop_alice = Transaction {
        initiator: Origin::Signed(alice.clone()),
        operation: Operation::Airdrop {
            receiver: alice.clone(),
            amount: alice_airdrop_amount,
        }
    };
    let airdrop_alice_result = Block::create_and_add(&mut suchain, vec![tx_airdrop_alice]);
    assert_eq!(Ok(()), airdrop_alice_result);
    println!("..Done\n\n");

    Ok(())
}
