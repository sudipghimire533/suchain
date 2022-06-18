use std::collections::HashMap;
use std::time::Duration;
use std::io::stdin;
use std::io::stdout;
use std::io::Write;

pub mod components;
pub mod chain;

use chain::ChainProperties;
use chain::Chain;
use components::consensus::Consensus;
use components::consensus::ProofOfWork;
use components::Balance;
use components::AccountId;
use components::transaction::Transaction;
use components::origin::Origin;
use components::transaction::Operation;
use components::block::Block;

use serde::Deserialize;
use serde::Serialize;

fn main() -> Result<(), i32> {
    println!(".\n.\n.\n");

    let mut node: Option<Chain> = None;
    loop {
        print!("\n>>");
        stdout().flush().expect("Error while printing to stdout..");
        let mut input = String::new();

        stdin().read_line(&mut input).expect("Error while reading input..");
        input = input.trim().to_string();
        if input.is_empty() || input.starts_with("//") {
            continue;
        }
        println!(">>{input}");

        let command = Command::construct(input);
        command.execute(&mut node);
        println!("");
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum Command {
    Clear,
    Exit,
    ShowNode,
    Help,
    NewNode {
        difficulty: usize,
        allowance: Balance,
        minimum_balance: Balance,
    },
    Operation(Transaction),
    Error(String),
    Unknown(String),
    IncreaseDifficulty(usize),
}

impl Command {
    fn construct(input: String) -> Self {
        let input = input.trim().to_string();
        let mut command = String::new();
        for ch in input.to_ascii_lowercase().as_str().chars() {
            if ch.is_whitespace() {
                break;
            }
            command.push(ch)
        }
        let rest = input.split_at(command.len()).1.to_string();

        match command.as_str() {
            "help" | "usage" | "how" => Command::Help,
            "quit" | "exit" | ":q" => Command::Exit,
            "show_node" | "show" => Command::ShowNode,
            "clear" | "cls" => Command::Clear,
            "new_node" | "create_node" => {
                match serde_json::from_str(format!("{{\"NewNode\": {rest}}}").as_str()) {
                    Ok(a) => a,
                    Err(err) => {
                        Command::Error(
                            format!("While parsing NewNode. Error: {:?}", err)
                        )

                    },
                }
            }
            "do_transaction" | "do_operation" => {
                match serde_json::from_str(format!("{{\"Operation\": {rest}}}").as_str()) {
                    Ok(a) => a,
                    Err(err) => {
                        Command::Error(
                            format!("While parsing Operation. Error: {:?}", err)
                        )

                    },
                }
            }
            "increase_difficulty" | "set_difficulty" => {
                let difficulty_cmd_res = rest.trim().parse::<usize>()
                    .map_err(|_| "Invalid difficulty paramater. Should have been a number")
                    .map(|new_difficulty| Command::IncreaseDifficulty(new_difficulty));
                match difficulty_cmd_res {
                    Ok(v) => v,
                    Err(e) => Command::Error(e.into()),
                }
            }
            cmd => {
                let convert_res = serde_json::from_str(input.as_str());
                match convert_res {
                    Ok(v) => v,
                    Err(_) => Command::Unknown(cmd.to_string()),
                }
            },
        }
    }

    fn execute(self, node: &mut Option<Chain>) {
        match self {
            Command::Clear => clear_screen(),
            Command::Exit => exit_program(),
            Command::ShowNode => show_node(node),
            Command::Help => show_help(),
            Command::NewNode { difficulty, allowance, minimum_balance} =>
                new_node(node, difficulty, allowance, minimum_balance),
            Command::Unknown(command) => unknown_command(&command),
            Command::Error(err) => println!("Error parsing comand: {err}"),
            Command::Operation(op) => perform_operation(node, op),
            Command::IncreaseDifficulty(difficulty) =>
                increase_difficulty(node, difficulty),
        }
    }
}

fn perform_operation(node_container: &mut Option<Chain>, transaction: Transaction) {
    match node_container {
        None => println!("No node loaded. Use new_node operation first"),
        Some(node) => {
            let mut block = Block::new(node);
            block.transactions.push(transaction);
 
            let prep_res = <ProofOfWork as Consensus>::prepare_block(node, &mut block);
            if let Err(prep_err) = prep_res {
                println!("While preparing block with this transaction. {prep_err:?}");
            }

            let add_res = node.add_block(block);
            if let Err(tx_err) = add_res {
                println!("Can not perform this transaction. While adding block Error: {tx_err:?}");
            }
        }
    }
}

fn increase_difficulty(node_container: &mut Option<Chain>, new_difficulty: usize) {
    match node_container {
        None => println!("No node loaded. Use new_node operation first"),
        Some(node) => {
            node.properties.difficulty = new_difficulty;
        }
    }
}

fn new_node(
    node_container: &mut Option<Chain>,
    difficulty: usize,
    allowance: Balance,
    exestinsial_deposit: Balance
){
    let prop = ChainProperties {
        exestinsial_deposit,
        difficulty,
        time_tolorant: Duration::from_secs(10),
        consensus: ProofOfWork
    };
    let info = "suchain v0.1 testnet";
    let chain = Chain::new(info.into(), prop, allowance);

    *node_container = Some(chain);
}

fn show_help() {
println!(r##"
    suchain help.
    Sytanx: command <options>[]

    Available command:
    - clear
          the screen

    - help
           print this help message

    - quit
           quit this program

    - new_node {{"info": STRING, "difficulty": Number, "allowance": Balance}}
           create a new node with given parameters
"##);
}

fn show_node(node: &mut Option<Chain>) {
    match node {
        None => println!("None"),
        Some(val) => println!("{val}")
    }
    
}

fn exit_program() {
    println!("\n.\n.\n\tAnd that's how a great era ended...\n\n");
    std::process::exit(0);
}

fn clear_screen() {
    println!("\x1b[H\x1b[J");
}

fn unknown_command(input: &str) {
    println!("\n Command: `{input}` is not valid. type help if you are lost");
}
