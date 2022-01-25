use std::env;
use colored::*;
use std;
use minparse::minparse;
pub mod encrypt_file;
pub mod symmetric;

const AUTHOR: &str = "@Octalbyte";
const VERSION: &str = "0.1.0";

fn main() {
    println!("Locker v{} made by {}", VERSION, AUTHOR);
   
    let subc = minparse::subcommands();
    match subc[1] {
        "encrypt" => {
            match subc[2] {
                "file" => {

                }

            }
        },
        "help" => {
            match subc[2] {
                "encrypt" => {}
                "decrypt" => {}
                "hash" => {}
                "check" => {}
            }
        }
    }

    //encrypt_file::encrypt_file::hello();
}
