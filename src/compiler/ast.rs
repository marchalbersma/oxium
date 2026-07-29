use crate::compiler::span::Span;

#[derive(Clone, Debug)]
pub struct File {
    pub decls: Vec<Decl>,
}

#[derive(Clone, Debug)]
pub enum Decl {
    Func(FuncDecl),
}

#[derive(Clone, Debug)]
pub struct FuncDecl {
    pub name: Ident,
    pub sig: FuncSig,
    pub body: BlockExpr,
    pub span: Span,
}

#[derive(Clone, Debug)]
pub enum Expr {
    Block(BlockExpr),
}

#[derive(Clone, Debug)]
pub struct BlockExpr {
    pub span: Span,
}

#[derive(Clone, Debug)]
pub struct FuncSig {
    pub span: Span,
}

#[derive(Clone, Debug)]
pub struct Ident {
    pub value: String,
    pub span: Span,
}
