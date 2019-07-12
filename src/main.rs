mod syntax;

use std::env;
use std::fs;
use syntax::parser::Parser;

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
            parser.parse(&src);
        },
        Err(e) => println!("Error: {:?}", e),
    }
}
