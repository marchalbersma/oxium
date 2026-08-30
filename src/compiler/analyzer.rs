use crate::compiler::ast::{Decl, File};
use crate::compiler::symbol::SymbolTable;

pub struct Analyzer {
    file: File,
    symbols: SymbolTable,
}

impl Analyzer {
    pub fn new(file: File) -> Self {
        Self {
            file,
            symbols: SymbolTable::default(),
        }
    }

    pub fn analyze(&mut self) -> SymbolTable {
        self.resolve_names();
        self.check_main_func();

        self.symbols.clone()
    }

    fn resolve_names(&mut self) {
        for decl in &self.file.decls {
            match decl {
                Decl::Extern(ext) => {
                    for func in &ext.funcs {
                        self.symbols.insert_func(true, func.clone());
                    }
                }
                Decl::Func(func) => {
                    self.symbols.insert_func(false, func.clone());
                }
            }
        }
    }

    fn check_main_func(&self) {
        if self.symbols.find_func_by_name("main").is_none() {
            panic!("Missing main function");
        }
    }
}
