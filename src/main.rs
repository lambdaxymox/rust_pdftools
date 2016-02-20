extern crate rust_pdftools;

use rust_pdftools::imagemagick::imagemagick_commands as commands;

fn main() {
    println!("{}", commands::imagemagick_identify_default(&"foo.tiff".to_string()).unwrap());
}