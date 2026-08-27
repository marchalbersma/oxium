use crate::compiler::asm::ast::{
    Block, Call, Directive, File, Immediate, Instruction, Mov, Operand, Register, Section,
    SectionKind, Sub, Symbol,
};

pub struct Generator;

impl Generator {
    pub fn generate(&self) -> File {
        let mut file = File {
            directives: Vec::new(),
            sections: Vec::new(),
        };

        self.generate_entry_point(&mut file);

        file
    }

    fn generate_entry_point(&self, file: &mut File) {
        let entry = Symbol {
            value: "__entry__".into(),
        };
        let exit_process = Symbol {
            value: "ExitProcess".into(),
        };

        file.directives.push(Directive::Global(entry.clone()));
        file.directives
            .push(Directive::Extern(exit_process.clone()));

        file.sections.push(Section {
            kind: SectionKind::Text,
            blocks: vec![Block {
                label: entry,
                instructions: vec![
                    Instruction::Sub(Sub {
                        dest: Operand::Register(Register::Rsp),
                        src: Operand::Immediate(Immediate::Signed(40)),
                    }),
                    Instruction::Mov(Mov {
                        dest: Operand::Register(Register::Rcx),
                        src: Operand::Immediate(Immediate::Signed(0)),
                    }),
                    Instruction::Call(Call {
                        target: Operand::Symbol(exit_process),
                    }),
                ],
            }],
        })
    }
}
