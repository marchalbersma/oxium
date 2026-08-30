use crate::compiler::span::Span;

#[derive(Clone, Debug)]
pub struct File {
    pub decls: Vec<Decl>,
}

#[derive(Clone, Debug)]
pub enum Decl {
    Extern(ExternDecl),
    Func(FuncDecl),
}

#[derive(Clone, Debug)]
pub struct ExternDecl {
    pub funcs: Vec<FuncDecl>,
    pub span: Span,
}

#[derive(Clone, Debug)]
pub struct FuncDecl {
    pub name: Ident,
    pub sig: FuncSig,
    pub body: Option<BlockExpr>,
    pub span: Span,
}

#[derive(Clone, Debug)]
pub enum Expr {
    Block(BlockExpr),
    Call(CallExpr),
}

#[derive(Clone, Debug)]
pub struct BlockExpr {
    pub expressions: Vec<Expr>,
    pub span: Span,
}

#[derive(Clone, Debug)]
pub struct CallExpr {
    pub name: Ident,
    pub args: Vec<Lit>,
    pub span: Span,
}

#[derive(Clone, Debug)]
pub struct FuncSig {
    pub params: Vec<Param>,
    pub span: Span,
}

#[derive(Clone, Debug)]
pub struct Param {
    pub name: Ident,
    pub ty: Ident,
    pub span: Span,
}

#[derive(Clone, Debug)]
pub struct Ident {
    pub value: String,
    pub span: Span,
}

#[derive(Clone, Debug)]
pub enum Lit {
    Int(Int),
}

#[derive(Clone, Debug)]
pub enum Int {
    Signed(i64),
    Unsigned(u64),
}
