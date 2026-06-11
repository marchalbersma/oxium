use crate::compiler::cursor::Cursor;
use std::fs;

pub mod compiler;

fn main() {
    let src = fs::read_to_string("examples/main/main.ox").expect("Failed to read source file");

    let mut cursor = Cursor::new(&src);

    loop {
        let peek = cursor.peek();

        if peek.is_none() {
            break;
        }

        println!(
            "peek: {}, pos: {}",
            (peek.unwrap() as char).escape_default(),
            cursor.pos,
        );

        println!(
            "next: {}, pos: {}",
            (cursor.next().unwrap() as char).escape_default(),
            cursor.pos,
        );
    }
}
