mod syntax;

use std::env;
use std::fs;
use syntax::parser::Parser;
use syntax::token::Token;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("Please provide source code filename!");
        return;
    }

    let filename = &args[1];
    match fs::read_to_string(filename) {
        Ok(src) => {
            let mut parser = Parser::new();
            let tokens: Vec<Token> = parser.parse(&src);
            for t in &tokens {
                println!("{:?}", t.literal);
            }
        },
        Err(e) => println!("Error: {:?}", e),
    }
}
