use crate::compiler::Compiler;

pub mod compiler;

fn main() {
    let compiler = Compiler::new("build", false);

    if let Err(error) = compiler.compile_and_run("examples/math") {
        eprintln!("Failed to compile: {}", error);
    }
}
