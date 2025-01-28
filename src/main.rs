use std::io;

use repl::start;

pub mod ast;
pub mod lexer;
pub mod parser;
pub mod repl;
pub mod token;

fn main() {
    println!("Salam! Bu oyrenmek ucun yazdigim interpereterdir!");
    println!("Feel free to type in the code");
    start(io::stdin(), io::stdout());
}
