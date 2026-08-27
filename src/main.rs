use crate::compiler::analyzer::Analyzer;
use crate::compiler::asm::generator::Generator;
use crate::compiler::asm::writer::Writer;
use crate::compiler::parser::Parser;
use std::fs;

pub mod compiler;

fn main() {
    let src = fs::read_to_string("examples/main/main.ox").expect("Failed to read source file");

    let mut parser = Parser::new(&src);
    let file = parser.parse();

    println!("AST: {:?}", file);

    let mut analyzer = Analyzer::new(file);
    let symbols = analyzer.analyze();

    println!("Functions: {:?}", symbols.funcs());

    let asm_generator = Generator;
    let asm_file = asm_generator.generate();

    println!("Assembly AST: {:?}", asm_file);

    let asm = Writer::write(&asm_file);

    println!("Assembly: {:?}", asm);

    let build_dir = "build";

    if !fs::exists(build_dir).expect("Failed to check if build directory exists") {
        fs::create_dir(build_dir).expect("Failed to create build directory");
    }

    fs::write(format!("{}/main.asm", build_dir), asm).expect("Failed to write assembly file");
}
