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

fn main() -> Result<(), i32> {
    println!(".\n.\n.\n.\n.\n");

    let mut node: Option<Chain> = None;

    'input_loop: loop {
        print!("\n>>");
        stdout().flush().expect("Error while printing to stdout..");
        let mut input = String::new();

        stdin().read_line(&mut input).expect("Error while reading input..");
        input = input.trim().to_string();
        match input.as_str() {
            "quit" | "exit" => break 'input_loop,
            "clear" => clear_screen(),
            "help" => show_help(),
            "show_node" => show_node(&node),

            _ => unknown_command(input),
        }
    }

    println!("\n.\n.\n.\n\tAnd that's how a great era ended...\n\n");
    Ok(())
}

fn clear_screen() {
    println!("\x1b[H\x1b[J");
}

fn unknown_command(input: String) {
    println!("\n Command: `{input}` is not valid. type help if you are lost");
}

fn show_help() {
println!(r##"
    suchain help.
    Sytanx: command <options>[]

    Available command:
    - clear: clear the screen
    - help: print this help message
    - quit: quit this program
"##);
}


fn show_node<D: core::fmt::Debug>(node: D) {
    println!("{node:#?}");
}
