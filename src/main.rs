use std::fs;
use std::io::prelude::*;
use std::process;

use basic_compiler::{code_generator, parser, tokenise};

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 3 {
        eprintln!("Expected 2 argument recieved {}", args.len() - 1);
        process::exit(1);
    }
    let input = fs::read_to_string(&args[1]).unwrap_or_else(|_| {
        eprintln!("Invalid file path");
        process::exit(1)
    });

    let tokens = tokenise(input).unwrap_or_else(|e| {
        eprintln!("Error {e}");
        process::exit(1)
    });

    let ast = parser(0, tokens).unwrap_or_else(|e| {
        eprintln!("Error {e}");
        process::exit(1)
    });
    let program = code_generator(ast.node).unwrap_or_else(|e| {
        eprintln!("Error {e}");
        process::exit(1)
    });

    let mut file = fs::File::create(&args[2]).unwrap_or_else(|e| {
        eprintln!("Error {e}");
        process::exit(1)
    });

    file.write_all(program.as_bytes()).unwrap_or_else(|e| {
        eprintln!("Error {e}");
        process::exit(1)
    });
}
