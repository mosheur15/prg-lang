#![allow(unused)]
mod lexer;
mod token_data;

use std::{fs, process::exit};
use lexer::File;

fn readfile(filename: &str) -> String {
    match fs::read_to_string(filename) {
        Ok(content) => { return content.trim().to_string() }
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
