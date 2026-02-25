// Lisp parser

// Write code that takes some Lisp code and returns an abstract syntax tree.
// The AST should represent the structure of the code and the meaning of each token.
// For example, if your code is given (first (list 1 (+ 2 3) 9)),
// it could return a nested array like ["first", ["list", 1, ["+", 2, 3], 9]].

use crate::{parser::Parser, scanner::Scanner};
use std::env;

mod parser;
mod scanner;

fn main() {
    let args: Vec<String> = env::args().collect();
    let code = if args.len() > 1 {
        args[1].clone()
    } else {
        eprintln!("Usage: lisp-parser <code>");
        return;
    };
    let scanner = Scanner::new(&code);
    let tokens = scanner.scan();
    let mut parser = Parser::new(tokens);
    println!("{}", parser.parse())
}
