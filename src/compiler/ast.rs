use crate::compiler::span::Span;

#[derive(Debug)]
pub struct File {
    pub decls: Vec<Decl>,
}

#[derive(Debug)]
pub enum Decl {
    Func(FuncDecl),
}

#[derive(Debug)]
pub struct FuncDecl {
    pub name: Ident,
    pub sig: FuncSig,
    pub body: BlockExpr,
    pub span: Span,
}

#[derive(Debug)]
pub enum Expr {
    Block(BlockExpr),
}

#[derive(Debug)]
pub struct BlockExpr {
    pub span: Span,
}

#[derive(Debug)]
pub struct FuncSig {
    pub span: Span,
}

#[derive(Debug)]
pub struct Ident {
    pub value: String,
    pub span: Span,
}
