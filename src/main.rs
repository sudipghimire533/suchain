use std::collections::HashMap;
use std::time::Duration;
use std::io::stdin;
use std::io::stdout;
use std::io::Write;

pub mod components;
pub mod chain;

use chain::ChainProperties;
use chain::Chain;
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
    println!(".\n.\n.\n.\n.\n");

    let mut node: Option<Chain> = None;

    loop {
        print!("\n>>");
        stdout().flush().expect("Error while printing to stdout..");
        let mut input = String::new();

        stdin().read_line(&mut input).expect("Error while reading input..");
        input = input.trim().to_string();

        let command = Command::construct(input);
        command.execute(&mut node);
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
    Error(String),
    Unknown(String),
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
                let cmd = format!("{{\"NewNode\": {rest}}}");
                let res = serde_json::from_str(cmd.as_str())
                    .map_err(|err|{
                        Command::Error(
                            format!("While parsing NewNode. Error: {:?}", err)
                        )
                    });
                match res {
                    Ok(a) => a,
                    Err(a) => a,
                }
            }
            cmd => Command::Unknown(cmd.to_string()),
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
        Some(val) => println!("{val:#?}")
    }
    
}

fn exit_program() {
    println!("\n.\n.\n.\n\tAnd that's how a great era ended...\n\n");
    std::process::exit(0);
}

fn clear_screen() {
    println!("\x1b[H\x1b[J");
}

fn unknown_command(input: &str) {
    println!("\n Command: `{input}` is not valid. type help if you are lost");
}
