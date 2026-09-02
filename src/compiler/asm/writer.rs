use crate::compiler::asm::ast::{
    Block, Directive, File, Immediate, Instruction, Operand, Register, Section, SectionKind,
};

pub struct Writer;

impl Writer {
    pub fn write(file: &File) -> String {
        let mut asm = String::new();

        for directive in &file.directives {
            asm.push_str(&Self::write_directive(directive));
        }

        for section in &file.sections {
            asm.push_str(&Self::write_section(section));
        }

        asm
    }

    fn write_directive(directive: &Directive) -> String {
        match directive {
            Directive::Global(symbol) => format!("global {}\n\n", symbol.value),
            Directive::Extern(symbol) => format!("extern {}\n\n", symbol.value),
        }
    }

    fn write_section(section: &Section) -> String {
        let mut asm = String::new();

        let name = match section.kind {
            SectionKind::Text => "text",
        };

        asm.push_str(&format!("section .{}\n\n", name));

        let blocks = section
            .blocks
            .iter()
            .map(Self::write_block)
            .collect::<Vec<_>>()
            .join("\n");

        asm.push_str(&blocks);

        asm
    }

    fn write_block(block: &Block) -> String {
        let mut asm = String::new();

        asm.push_str(&format!("{}:\n", block.label.value));

        for instruction in &block.instructions {
            asm.push_str(&format!("    {}\n", Self::write_instruction(instruction)));
        }

        asm
    }

    fn write_instruction(instruction: &Instruction) -> String {
        match instruction {
            Instruction::Call(call) => {
                format!("call {}", Self::write_operand(&call.target))
            }
            Instruction::Mov(mov) => {
                format!(
                    "mov {}, {}",
                    Self::write_operand(&mov.dest),
                    Self::write_operand(&mov.src),
                )
            }
            Instruction::Add(add) => {
                format!(
                    "add {}, {}",
                    Self::write_operand(&add.dest),
                    Self::write_operand(&add.src),
                )
            }
            Instruction::Sub(sub) => {
                format!(
                    "sub {}, {}",
                    Self::write_operand(&sub.dest),
                    Self::write_operand(&sub.src),
                )
            }
            Instruction::Ret => "ret".to_string(),
        }
    }

    fn write_operand(operand: &Operand) -> String {
        match operand {
            Operand::Immediate(immediate) => Self::write_immediate(immediate),
            Operand::Register(register) => Self::write_register(register),
            Operand::Symbol(symbol) => symbol.value.clone(),
        }
    }

    fn write_immediate(immediate: &Immediate) -> String {
        match immediate {
            Immediate::Signed(value) => value.to_string(),
            Immediate::Unsigned(value) => value.to_string(),
        }
    }

    fn write_register(register: &Register) -> String {
        match register {
            Register::Rcx => "rcx".to_string(),
            Register::Rsp => "rsp".to_string(),
        }
    }
}
