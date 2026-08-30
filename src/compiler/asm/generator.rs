use crate::compiler::asm::ast::{
    Block, Call, Directive, File, Immediate, Instruction, Mov, Operand, Register, Section,
    SectionKind, Sub, Symbol,
};
use crate::compiler::ast::{Expr, Int, Lit};
use crate::compiler::symbol::{FuncSymbol, SymbolTable};

pub struct Generator {
    symbols: SymbolTable,
}

impl Generator {
    const ENTRY: &str = "__entry__";

    pub fn new(symbols: SymbolTable) -> Self {
        Self { symbols }
    }

    pub fn generate(&self) -> File {
        let mut file = File {
            directives: Vec::new(),
            sections: Vec::new(),
        };

        self.generate_global_directives(&mut file);
        self.generate_extern_directives(&mut file);

        self.generate_entry_point(&mut file);

        file
    }

    fn generate_global_directives(&self, file: &mut File) {
        let symbol = Symbol {
            value: Self::ENTRY.into(),
        };

        file.directives.push(Directive::Global(symbol));
    }

    fn generate_extern_directives(&self, file: &mut File) {
        let funcs: Vec<&FuncSymbol> = self
            .symbols
            .funcs()
            .iter()
            .filter(|func| func.ext)
            .collect();

        for func in funcs {
            let symbol = Symbol {
                value: func.decl.name.value.clone(),
            };

            file.directives.push(Directive::Extern(symbol));
        }
    }

    fn generate_entry_point(&self, file: &mut File) {
        let mut instructions = vec![Instruction::Sub(Sub {
            dest: Operand::Register(Register::Rsp),
            src: Operand::Immediate(Immediate::Signed(40)),
        })];

        let main_func: &FuncSymbol = self
            .symbols
            .funcs()
            .iter()
            .find(|func| func.decl.name.value == "main")
            .unwrap();

        if let Some(body) = &main_func.decl.body {
            for expr in &body.expressions {
                match expr {
                    Expr::Call(call) => {
                        for arg in &call.args {
                            let src = match arg {
                                Lit::Int(int) => match int {
                                    Int::Signed(value) => {
                                        Operand::Immediate(Immediate::Signed(*value))
                                    }
                                    Int::Unsigned(value) => {
                                        Operand::Immediate(Immediate::Unsigned(*value))
                                    }
                                },
                            };

                            instructions.push(Instruction::Mov(Mov {
                                dest: Operand::Register(Register::Rcx),
                                src,
                            }))
                        }

                        instructions.push(Instruction::Call(Call {
                            target: Operand::Symbol(Symbol {
                                value: call.name.value.clone(),
                            }),
                        }))
                    }
                    expr => panic!("Unsupported expression: {:?}", expr),
                }
            }
        }

        file.sections.push(Section {
            kind: SectionKind::Text,
            blocks: vec![Block {
                label: Symbol {
                    value: Self::ENTRY.into(),
                },
                instructions,
            }],
        })
    }
}
