use std::io::{self, Read};
include!(concat!(env!("OUT_DIR"), "/elements.rs"));

use brba_text::process_text;

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).expect("Failed to read stdin");

    let bytes = input.as_bytes();

    println!("{}", process_text(bytes));
}
