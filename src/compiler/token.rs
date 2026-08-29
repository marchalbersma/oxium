use crate::compiler::span::Span;

#[derive(Debug, Eq, PartialEq)]
pub enum TokenKind<'a> {
    Extern,
    Func,
    Ident(&'a str),
    Int(Int),
    OpenBrace,
    CloseBrace,
    OpenParen,
    CloseParen,
    Colon,
    Comma,
    Newline,
}

#[derive(Debug, Eq, PartialEq)]
pub enum Int {
    Signed(i64),
    Unsigned(u64),
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
