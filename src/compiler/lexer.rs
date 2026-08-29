use crate::compiler::cursor::Cursor;
use crate::compiler::span::Span;
use crate::compiler::token::{Int, Token, TokenKind};

pub struct Lexer<'a> {
    src: &'a str,
    cursor: Cursor<'a>,
}

impl<'a> Lexer<'a> {
    pub fn new(src: &'a str) -> Self {
        Self {
            src,
            cursor: Cursor::new(src),
        }
    }

    fn consume_while(&mut self, predicate: impl Fn(u8) -> bool) {
        while let Some(byte) = self.cursor.peek() {
            if !predicate(byte) {
                break;
            }

            self.cursor.next();
        }
    }

    fn is_ident_start(byte: u8) -> bool {
        byte.is_ascii_alphabetic()
    }

    fn is_ident_continue(byte: u8) -> bool {
        byte.is_ascii_alphanumeric() || byte == b'_'
    }

    fn is_horizontal_whitespace(byte: u8) -> bool {
        matches!(byte, b' ' | b'\r' | b'\t')
    }

    fn token(&self, kind: TokenKind<'a>, start: usize) -> Token<'a> {
        Token::new(kind, Span::new(start, self.cursor.pos))
    }
}

impl<'a> Iterator for Lexer<'a> {
    type Item = Token<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        self.consume_while(Self::is_horizontal_whitespace);

        let start = self.cursor.pos;
        let byte = self.cursor.next()?;

        Some(match byte {
            b'{' => self.token(TokenKind::OpenBrace, start),

            b'}' => self.token(TokenKind::CloseBrace, start),

            b'(' => self.token(TokenKind::OpenParen, start),

            b')' => self.token(TokenKind::CloseParen, start),

            b':' => self.token(TokenKind::Colon, start),

            b',' => self.token(TokenKind::Comma, start),

            b'\n' => self.token(TokenKind::Newline, start),

            byte if byte == b'-' || byte.is_ascii_digit() => {
                self.consume_while(|byte| byte.is_ascii_digit());

                let str = &self.src[start..self.cursor.pos];

                let int = if byte == b'-' {
                    Int::Signed(str.parse::<i64>().unwrap())
                } else {
                    Int::Unsigned(str.parse::<u64>().unwrap())
                };

                self.token(TokenKind::Int(int), start)
            }

            byte if Self::is_ident_start(byte) => {
                self.consume_while(Self::is_ident_continue);

                let ident = &self.src[start..self.cursor.pos];

                match ident {
                    "extern" => self.token(TokenKind::Extern, start),
                    "func" => self.token(TokenKind::Func, start),
                    _ => self.token(TokenKind::Ident(ident), start),
                }
            }

            _ => {
                panic!("Unexpected character {:?}", byte as char);
            }
        })
    }
}
