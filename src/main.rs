use crate::compiler::parser::Parser;
use std::fs;

pub mod compiler;

fn main() {
    let src = fs::read_to_string("examples/main/main.ox").expect("Failed to read source file");

    let mut parser = Parser::new(&src);

    let file = parser.parse();

    println!("{file:?}");
}
