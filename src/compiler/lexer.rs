use crate::compiler::lexer::token::{Token, TokenKind};
use crate::compiler::span::Span;

pub mod token;

pub struct Lexer<'a> {
    src: &'a str,
    peeked: Option<Token>,
    token_start: usize,
    pos: usize,
}

impl<'a> Lexer<'a> {
    pub fn new(src: &'a str) -> Self {
        Self {
            src,
            peeked: None,
            token_start: 0,
            pos: 0,
        }
    }

    pub fn next_token(&mut self) -> Token {
        match self.peeked.take() {
            Some(token) => token,
            None => self.lex_token(),
        }
    }

    pub fn peek_kind(&mut self) -> &TokenKind {
        if self.peeked.is_none() {
            let token = self.lex_token();
            self.peeked = Some(token);
        }

        &self.peeked.as_ref().unwrap().kind
    }

    fn lex_token(&mut self) -> Token {
        self.consume_while(Self::is_horizontal_whitespace);

        self.token_start = self.pos;

        let kind = if let Some(byte) = self.next_byte() {
            match byte {
                byte if Self::is_ident_start(byte) => self.lex_keyword_or_ident(),
                byte if Self::is_numeric(byte) => self.lex_number(),
                b'{' => TokenKind::OpenBrace,
                b'}' => TokenKind::CloseBrace,
                b'(' => TokenKind::OpenParen,
                b')' => TokenKind::CloseParen,
                b':' => TokenKind::Colon,
                b',' => TokenKind::Comma,
                b'\n' => TokenKind::Newline,
                _ => TokenKind::Unknown,
            }
        } else {
            TokenKind::Eof
        };

        self.token(kind)
    }

    fn lex_keyword_or_ident(&mut self) -> TokenKind {
        self.consume_while(Self::is_ident_continue);

        match self.lexeme() {
            "extern" => TokenKind::Extern,
            "func" => TokenKind::Func,
            _ => TokenKind::Ident,
        }
    }

    fn lex_number(&mut self) -> TokenKind {
        self.consume_while(Self::is_numeric);

        TokenKind::Int
    }

    fn is_ident_start(byte: u8) -> bool {
        byte == b'_' || byte.is_ascii_alphabetic()
    }

    fn is_ident_continue(byte: u8) -> bool {
        byte == b'_' || byte.is_ascii_alphanumeric()
    }

    fn is_horizontal_whitespace(byte: u8) -> bool {
        matches!(byte, b' ' | b'\r' | b'\t')
    }

    fn is_numeric(byte: u8) -> bool {
        byte.is_ascii_digit()
    }

    fn consume_while(&mut self, predicate: impl Fn(u8) -> bool) {
        while self.consume_if(&predicate) {}
    }

    fn consume_if(&mut self, predicate: impl Fn(u8) -> bool) -> bool {
        match self.peek_byte() {
            Some(byte) if predicate(byte) => {
                self.next_byte();
                true
            }
            _ => false,
        }
    }

    fn lexeme(&self) -> &str {
        &self.src[self.token_start..self.pos]
    }

    fn next_byte(&mut self) -> Option<u8> {
        let byte = self.peek_byte()?;
        self.pos += 1;

        Some(byte)
    }

    fn peek_byte(&self) -> Option<u8> {
        self.src.as_bytes().get(self.pos).copied()
    }

    fn span(&self) -> Span {
        Span {
            start: self.token_start,
            end: self.pos,
        }
    }

    fn token(&self, kind: TokenKind) -> Token {
        Token {
            kind,
            span: self.span(),
        }
    }
}
