use crate::compiler::span::Span;

#[derive(Debug, Eq, PartialEq)]
pub enum TokenKind<'a> {
    Func,
    Ident(&'a str),
    OpenBrace,
    CloseBrace,
    OpenParen,
    CloseParen,
    Newline,
}

#[derive(Debug)]
pub struct Token<'a> {
    pub kind: TokenKind<'a>,
    pub span: Span,
}

impl<'a> Token<'a> {
    pub fn new(kind: TokenKind<'a>, span: Span) -> Self {
        Self { kind, span }
    }
}
