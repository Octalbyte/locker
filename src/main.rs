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

    #[clap(short, long, default_value = "")]
    pwd: String,

    #[clap(short, long, default_value = "")]
    hash: String,
}

pub mod encrypt_file;
pub mod symmetric;

const AUTHOR: &str = "@Octalbyte";
const VERSION: &str = "0.1.0";

fn main() {
    println!("Locker v{} made by {}", VERSION, AUTHOR);
    let args: Vec<String> = env::args().collect();
    let clap_args = Args::parse();

    //encrypt_file::encrypt_file::hello();
}
