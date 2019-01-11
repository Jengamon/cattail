mod token;
mod lexer;
mod ast;
mod parser;

use crate::lexer::Lexer;
use crate::parser::Parser;

fn main() {
    let test_text = include_str!("../test.ctl");
    let mut lexer = Lexer::new(test_text.chars());
    let tokens = lexer.collect::<Vec<_>>();
    for tok in &tokens {
        if let Ok(ref t) = tok {
            println!("{:?}", t);
        } else if let Err(ref e) = tok {
            if !e.is_eof() {
                eprintln!("{:?}", e)
            }
        }
    }

    println!("=== PARSER OUTPUT ===");
    let mut parser = Parser::new(tokens.into_iter().filter(|x| x.is_ok()).map(|x| x.unwrap()).collect::<Vec<_>>());
    println!("{:#?}", parser.expression());
    println!("Hello, world!");
}
