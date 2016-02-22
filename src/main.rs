extern crate rust_pdftools;

use rust_pdftools::imagemagick::imagemagick_commands as commands;
use std::env;

fn usage() -> &'static str {
    "USAGE: cargo run \"path/to/file\""
}

fn main() {
    let args: Vec<_> = env::args().collect();

    if args.len() > 1 {
        println!("{}\n", commands::imagemagick_identify_default(&args[1].to_string()).unwrap()); 
    } else {
        println!("{}", usage());
    }
    
}