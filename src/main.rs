use crate::compiler::lexer::Lexer;
use std::fs;

pub mod compiler;

fn main() {
    let src = fs::read_to_string("examples/main/main.ox").expect("Failed to read source file");

    let lexer = Lexer::new(&src);

    for token in lexer {
        println!("{token:?}");
    }
}
