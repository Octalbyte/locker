use colored::*;
use minparse::minparse;
pub mod encrypt_file;
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
        "encrypt" => {
            match subc[2].as_str() {
                "file" => {

                }
                _ => {}

            }
        },
        "help" => {
            match subc[2].as_str() {
                "encrypt" => {}
                "decrypt" => {}
                "hash" => {}
                "check" => {}
                _ => {}
            }
        },

        _ => {}
    }

    //encrypt_file::encrypt_file::hello();
}
