extern crate rust_pdftools;

//use rust_pdftools::imagemagick::imagemagick_commands as commands;
use rust_pdftools::imagemagick::operations::ImageMagickOperation as IMO;
use rust_pdftools::imagemagick::operations as ops;
use rust_pdftools::image_tools::image_ops::RunOperation as RO;
use rust_pdftools::image_tools::image_ops::ElementaryPageOperations as EPO;
use std::env;

fn usage() -> &'static str {
    "USAGE: cargo run \"path/to/file\""
}

fn main() {
    let args: Vec<_> = env::args().collect();

    if args.len() > 1 {
        let op: IMO = EPO::identify(String::from(""), args[1].clone());
        let res = RO::run_operation(op);
        println!("Operation completed.");
        //println!("{}\n", RO::run_operation(op)); 
    } else {
        println!("{}", usage());
    }
    
}