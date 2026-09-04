use crate::compiler::parser::ast::FuncDecl;
use std::collections::HashMap;

#[derive(Clone, Default)]
pub struct SymbolTable {
    funcs: Vec<FuncSymbol>,
    func_map: HashMap<String, usize>,
}

impl SymbolTable {
    pub fn insert_func(&mut self, ext: bool, decl: FuncDecl) {
        if self.func_map.contains_key(&decl.name.value) {
            panic!("Duplicate function {:?}", decl.name.value);
        }

        let id = self.funcs.len();
        let name = decl.name.value.clone();

        self.funcs.push(FuncSymbol { id, ext, decl });
        self.func_map.insert(name, id);
    }

    pub fn funcs(&self) -> &Vec<FuncSymbol> {
        &self.funcs
    }

    pub fn find_func_by_name(&self, name: &str) -> Option<&FuncSymbol> {
        let id = self.func_map.get(name)?;
        Some(&self.funcs[*id])
    }
}

#[derive(Clone, Debug)]
pub struct FuncSymbol {
    pub id: usize,
    pub ext: bool,
    pub decl: FuncDecl,
}
