use crate::compiler::span::Span;

#[derive(Debug, Eq, PartialEq)]
pub enum TokenKind {
    Extern,
    Func,
    Ident,
    Int,
    OpenBrace,
    CloseBrace,
    OpenParen,
    CloseParen,
    Colon,
    Comma,
    Newline,
    Unknown,
    Eof,
}

#[derive(Debug)]
pub struct Token {
    pub kind: TokenKind,
    pub span: Span,
}
