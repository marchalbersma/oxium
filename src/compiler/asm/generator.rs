use crate::compiler::asm::ast::{
    Add, Block, Call, Directive, File, Immediate, Instruction, Mov, Operand, Register, Section,
    SectionKind, Sub, Symbol,
};
use crate::compiler::parser::ast::{BlockExpr, CallExpr, Expr, FuncDecl, Int, Stmt};
use crate::compiler::symbol::{FuncSymbol, SymbolTable};
use std::vec;

pub struct Generator {
    symbols: SymbolTable,
}

impl Generator {
    const ENTRY: &str = "__entry__";

    pub fn new(symbols: SymbolTable) -> Self {
        Self { symbols }
    }

    pub fn generate(&self) -> File {
        File {
            directives: self.directives(),
            sections: self.sections(),
        }
    }

    fn directives(&self) -> Vec<Directive> {
        let mut directives = Vec::new();

        directives.push(self.entry_directive());

        let extern_funcs: Vec<&FuncSymbol> = self
            .symbols
            .funcs()
            .iter()
            .filter(|func| func.ext)
            .collect();

        for func in extern_funcs {
            directives.push(self.extern_directive(&func.decl));
        }

        directives
    }

    fn entry_directive(&self) -> Directive {
        Directive::Global(Symbol {
            value: Self::ENTRY.into(),
        })
    }

    fn extern_directive(&self, func: &FuncDecl) -> Directive {
        Directive::Extern(Symbol {
            value: func.name.value.clone(),
        })
    }

    fn sections(&self) -> Vec<Section> {
        vec![self.text_section()]
    }

    fn text_section(&self) -> Section {
        let kind = SectionKind::Text;
        let mut blocks = Vec::new();

        blocks.push(self.entry_block());

        let funcs: Vec<&FuncSymbol> = self
            .symbols
            .funcs()
            .iter()
            .filter(|func| !func.ext)
            .collect();

        for func in funcs {
            blocks.push(self.func_block(&func.decl));
        }

        Section { kind, blocks }
    }

    fn entry_block(&self) -> Block {
        let mut instructions = Vec::new();

        instructions.append(&mut self.func_prologue());
        instructions.push(Instruction::Call(Call {
            target: Operand::Symbol(Symbol {
                value: "main".to_string(),
            }),
        }));

        Block {
            label: Symbol {
                value: Self::ENTRY.into(),
            },
            instructions,
        }
    }

    fn func_block(&self, func: &FuncDecl) -> Block {
        let label = Symbol {
            value: func.name.value.clone(),
        };

        let mut instructions = Vec::new();

        instructions.append(&mut self.func_prologue());

        if let Some(body) = &func.body {
            instructions.append(&mut self.block_expr(body));
        }

        instructions.append(&mut self.func_epilogue());
        instructions.push(Instruction::Ret);

        Block {
            label,
            instructions,
        }
    }

    fn func_prologue(&self) -> Vec<Instruction> {
        vec![Instruction::Sub(Sub {
            dest: Operand::Register(Register::Rsp),
            src: Operand::Immediate(Immediate::Unsigned(40)),
        })]
    }

    fn func_epilogue(&self) -> Vec<Instruction> {
        vec![Instruction::Add(Add {
            dest: Operand::Register(Register::Rsp),
            src: Operand::Immediate(Immediate::Unsigned(40)),
        })]
    }

    fn block_expr(&self, block: &BlockExpr) -> Vec<Instruction> {
        let mut instructions = Vec::new();

        for stmt in &block.stmts {
            instructions.append(&mut match stmt {
                Stmt::Expr(expr) => match expr {
                    Expr::Call(call) => self.call_expr(call),
                    expr => panic!("Expected Call, found {:?}", expr),
                },
            });
        }

        instructions
    }

    fn call_expr(&self, call: &CallExpr) -> Vec<Instruction> {
        let mut instructions = Vec::new();

        for arg in &call.args {
            let src = match arg {
                Expr::Ident(_) => None,
                Expr::Int(int) => Some(match int {
                    Int::Signed(value) => Operand::Immediate(Immediate::Signed(*value)),
                    Int::Unsigned(value) => Operand::Immediate(Immediate::Unsigned(*value)),
                }),
                expr => panic!("Expected Ident or Int, found {:?}", expr),
            };

            if let Some(src) = src {
                instructions.push(Instruction::Mov(Mov {
                    dest: Operand::Register(Register::Rcx),
                    src,
                }))
            }
        }

        instructions.push(Instruction::Call(Call {
            target: Operand::Symbol(Symbol {
                value: call.name.value.clone(),
            }),
        }));

        instructions
    }
}
