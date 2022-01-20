use std::env;
use colored::*;
use std;
use clap::Parser;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Name of the person to greet
    #[clap(short, long, default_value="")]
    file: String,

    /// Number of times to greet
    #[clap(short, long, default_value = "")]
    string: String,
}

pub mod encrypt_file;
pub mod symmetric;

const AUTHOR: &str = "@Octalbyte";
const VERSION: &str = "0.1.0";

fn main() {
    println!("Locker v{} made by {}", VERSION, AUTHOR);
    let args: Vec<String> = env::args().collect();
    let clap_args = Args::parse();
    if 1 == args.len(){
        println!("At least one subcommand required");
        std::process::exit(1);
    }
    let command = &args[1];
    match command.as_str() {
        "encrypt" => {

        }
        _ => {
            println!("Command not found: {}", command.bold());
            println!("Run `locker --help` to get help");
            std::process::exit(1);
        }
    }
    //encrypt_file::encrypt_file::hello();
}
