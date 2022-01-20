use std::env;
use colored::*;
use std;

pub mod encrypt_file;
pub mod symmetric;

const AUTHOR: &str = "@Octalbyte";
const VERSION: &str = "0.1.0";

fn main() {
    println!("Locker v{} made by {}", VERSION, AUTHOR);
    let args: Vec<String> = env::args().collect();
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
