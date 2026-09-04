use crate::compiler::analyzer::Analyzer;
use crate::compiler::asm::generator::Generator;
use crate::compiler::asm::writer::Writer;
use crate::compiler::parser::Parser;
use crate::compiler::symbol::SymbolTable;
use parser::ast::File;
use std::error::Error;
use std::fs;
use std::path::Path;
use std::process::Command;

pub mod analyzer;
pub mod asm;
pub mod lexer;
pub mod parser;
pub mod span;
pub mod symbol;

pub struct Compiler<'a> {
    build_dir: &'a str,
    debug: bool,
}

impl<'a> Compiler<'a> {
    pub fn new(build_dir: &'a str, debug: bool) -> Self {
        Self { build_dir, debug }
    }

    pub fn compile_and_run(&self, src_dir: &str) -> Result<(), Box<dyn Error>> {
        let name = self.extract_name(src_dir);

        let src_path = format!("{}/main.ox", src_dir);
        let asm_path = format!("{}/{}.asm", self.build_dir, name);
        let obj_path = format!("{}/{}.obj", self.build_dir, name);
        let exe_path = format!("{}/{}.exe", self.build_dir, name);

        let src = fs::read_to_string(src_path)?;

        let file = self.parse(&src);
        let symbols = self.analyze(file);

        let asm = self.generate_asm(symbols);

        fs::create_dir_all(self.build_dir)?;
        fs::write(&asm_path, asm)?;

        self.assemble(&asm_path, &obj_path)?;
        self.link(&obj_path, &exe_path)?;

        self.run(&exe_path)?;

        Ok(())
    }

    fn extract_name<'b>(&self, src_dir: &'b str) -> &'b str {
        Path::new(src_dir)
            .file_name()
            .and_then(|name| name.to_str())
            .unwrap()
    }

    fn parse(&self, src: &str) -> File {
        let mut parser = Parser::new(src);
        let file = parser.parse();

        if self.debug {
            println!("AST: {:?}\n", file);
        }

        file
    }

    fn analyze(&self, file: File) -> SymbolTable {
        let mut analyzer = Analyzer::new(file);
        let symbols = analyzer.analyze();

        if self.debug {
            println!("Functions: {:?}\n", symbols.funcs(),);
        }

        symbols
    }

    fn generate_asm(&self, symbols: SymbolTable) -> String {
        let generator = Generator::new(symbols);
        let file = generator.generate();

        if self.debug {
            println!("Assembly AST: {:?}\n", file);
        }

        Writer::write(&file)
    }

    fn assemble(&self, asm_path: &str, obj_path: &str) -> Result<(), Box<dyn Error>> {
        Command::new("bin/nasm/nasm")
            .arg(asm_path)
            .arg("-o")
            .arg(obj_path)
            .arg("-f")
            .arg("win64")
            .output()
            .map_err(|error| format!("Failed to assemble: {}", error))?;

        Ok(())
    }

    fn link(&self, obj_path: &str, exe_path: &str) -> Result<(), Box<dyn Error>> {
        Command::new("bin/lld/lld-link")
            .arg(obj_path)
            .arg(format!("-out:{}", exe_path))
            .arg("-entry:__entry__")
            .arg("-subsystem:console")
            .arg("-defaultlib:kernel32.lib")
            .output()
            .map_err(|error| format!("Failed to link: {}", error))?;

        Ok(())
    }

    fn run(&self, exe_path: &str) -> Result<(), Box<dyn Error>> {
        let output = Command::new(exe_path).output()?;
        println!("Exit code: {}", output.status.code().unwrap());

        Ok(())
    }
}
