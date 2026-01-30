#![allow(unused)]
mod lexer;

use std::{fs, process::exit};
use lexer::File;

fn readfile(filename: &str) -> Vec<u8> {
    match fs::read(filename) {
        Ok(content) => content,
        Err(e) => {
            println!("Cannot read '{}' : {}", filename, e.to_string());
            exit(404);
        }
    }
}


fn main() {
    let filename = "./src/test.txt";
    let mut file = File::new(readfile(filename));
    file.tokenize();
}
