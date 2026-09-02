#[derive(Clone, Debug)]
pub struct File {
    pub directives: Vec<Directive>,
    pub sections: Vec<Section>,
}

#[derive(Clone, Debug)]
pub enum Directive {
    Global(Symbol),
    Extern(Symbol),
}

#[derive(Clone, Debug)]
pub struct Section {
    pub kind: SectionKind,
    pub blocks: Vec<Block>,
}

#[derive(Clone, Debug)]
pub enum SectionKind {
    Text,
}

#[derive(Clone, Debug)]
pub struct Block {
    pub label: Symbol,
    pub instructions: Vec<Instruction>,
}

#[derive(Clone, Debug)]
pub enum Instruction {
    Call(Call),
    Mov(Mov),
    Add(Add),
    Sub(Sub),
    Ret,
}

#[derive(Clone, Debug)]
pub struct Call {
    pub target: Operand,
}

#[derive(Clone, Debug)]
pub struct Mov {
    pub dest: Operand,
    pub src: Operand,
}

#[derive(Clone, Debug)]
pub struct Add {
    pub dest: Operand,
    pub src: Operand,
}

#[derive(Clone, Debug)]
pub struct Sub {
    pub dest: Operand,
    pub src: Operand,
}

#[derive(Clone, Debug)]
pub enum Operand {
    Immediate(Immediate),
    Register(Register),
    Symbol(Symbol),
}

#[derive(Clone, Debug)]
pub enum Immediate {
    Signed(i64),
    Unsigned(u64),
}

#[derive(Clone, Debug)]
pub enum Register {
    Rcx,
    Rsp,
}

#[derive(Clone, Debug)]
pub struct Symbol {
    pub value: String,
}
