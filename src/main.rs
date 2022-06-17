use colored::*;
use minparse::minparse;
pub mod symmetric;
use std;
const AUTHOR: &str = "@Octalbyte";
const VERSION: &str = "0.1.0";

fn main() {
    println!("Locker v{} made by {}", VERSION.bold(), AUTHOR.bold());

    let subc = minparse::subcommands();
    if subc.len() == 1 {
        println!("");
        std::process::exit(0);
    }
    match subc[1].as_str() {
        "entry" => {
            let entry = subc[2].as_str();
            
	    }
        "help" => {
             println!("
                Welcome to Locker@0.1.0 by @Octalbyte

                Usage:

                locker entry [name] -> Add a password
                locker delete [name] -> Delete an entry
                locker copy [path] -> Copy to another path, together with the encrypted passwords
             ")
        },

        _ => {}
    }

    //encrypt_file::encrypt_file::hello();
}
