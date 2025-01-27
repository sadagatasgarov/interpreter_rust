use std::io;

use repl::start;

pub mod lexer;
pub mod token;
pub mod repl;
pub mod ast;

fn main() {
    println!("Salam! Bu oyrenmek ucun yazdigim interpereterdir!");
    println!("Feel free to type in the code");
    start(io::stdin(), io::stdout());
}
