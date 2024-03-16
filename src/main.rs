use std::process;

use basic_compiler::tokenise;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        eprintln!("Expected 1 argument recieved {}", args.len() - 1);
        process::exit(1);
    }
    let input = std::fs::read_to_string(&args[1]).unwrap_or_else(|_| {
       eprintln!("Invalid file path");
       process::exit(1)
    });

    let tokens = tokenise(input);

    println!("{:?}",tokens)
        
}
