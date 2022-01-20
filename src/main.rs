extern crate clap;

use std::env;
use colored::*;
use std;
use clap::{
    App,
    SubCommand,
    Arg
};

pub mod encrypt_file;
pub mod symmetric;

const AUTHOR: &str = "@Octalbyte";
const VERSION: &str = "0.1.0";

fn main() {
    println!("Locker v{} made by {}", VERSION, AUTHOR);
    let clap_args = App::new("locker")
        .subcommand(SubCommand::with_name("info")
            .arg(Arg::with_name("verbose")
                .short("v")
                .long("verbose"))
            .arg(Arg::with_name("PARAM")))
        .subcommand(SubCommand::with_name("sync")
            .arg(Arg::with_name("encrypt")
                .short("e")
                .long("encrypt")))
        //.default_subcommand("info")    // new method
        .get_matches();


    //encrypt_file::encrypt_file::hello();
}
