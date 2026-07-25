use crate::compiler::ast::{BlockExpr, Decl, File, FuncDecl, FuncSig, Ident};
use crate::compiler::lexer::Lexer;
use crate::compiler::span::Span;
use crate::compiler::token::{Token, TokenKind};

pub struct Parser<'a> {
    lexer: Lexer<'a>,
    current: Option<Token<'a>>,
}

impl<'a> Parser<'a> {
    pub fn new(src: &'a str) -> Self {
        let mut lexer = Lexer::new(src);
        let current = lexer.next();

        Self { lexer, current }
    }

    pub fn parse(&mut self) -> File {
        let mut decls = Vec::new();

        self.skip_newlines();

        while self.current.is_some() {
            decls.push(self.parse_decl());

            self.skip_newlines();
        }

        File { decls }
    }

    fn parse_decl(&mut self) -> Decl {
        match self.kind() {
            Some(TokenKind::Func) => Decl::Func(self.parse_func_decl()),

            kind => panic!("Expected Decl, found {:?}", kind),
        }
    }

    fn parse_func_decl(&mut self) -> FuncDecl {
        let func = self.expect(TokenKind::Func);
        let name = self.parse_ident();
        let sig = self.parse_func_sig();
        let body = self.parse_block_expr();

        let span = func.span.join(body.span);

        FuncDecl {
            name,
            sig,
            body,
            span,
        }
    }

    fn parse_block_expr(&mut self) -> BlockExpr {
        let open_brace = self.expect(TokenKind::OpenBrace);
        let close_brace = self.expect(TokenKind::CloseBrace);

        BlockExpr {
            span: open_brace.span.join(close_brace.span),
        }
    }

    fn parse_func_sig(&mut self) -> FuncSig {
        let open_paren = self.expect(TokenKind::OpenParen);
        let close_paren = self.expect(TokenKind::CloseParen);

        FuncSig {
            span: open_paren.span.join(close_paren.span),
        }
    }

    fn parse_ident(&mut self) -> Ident {
        let (value, span) = self.expect_ident();

        Ident { value, span }
    }

    fn expect(&mut self, expected: TokenKind<'a>) -> Token<'a> {
        let token = self.consume();

        if token.kind != expected {
            panic!("Expected {:?}, found {:?}", expected, token.kind);
        }

        token
    }

    fn expect_ident(&mut self) -> (String, Span) {
        let token = self.consume();

        match token.kind {
            TokenKind::Ident(name) => (name.to_owned(), token.span),

            kind => panic!("Expected Ident, found {:?}", kind),
        }
    }

    fn skip_newlines(&mut self) {
        self.skip_while(|token| matches!(token.kind, TokenKind::Newline));
    }

    fn skip_while(&mut self, predicate: impl Fn(&Token<'a>) -> bool + Copy) {
        while self.skip_if(predicate) {}
    }

    fn skip_if(&mut self, predicate: impl Fn(&Token<'a>) -> bool) -> bool {
        match self.current.as_ref() {
            Some(token) if predicate(token) => {
                self.consume();

                true
            }
            _ => false,
        }
    }

    fn consume(&mut self) -> Token<'a> {
        let current = self.current.take().expect("Unexpected end of file");

        self.current = self.lexer.next();

        current
    }

    fn kind(&self) -> Option<&TokenKind<'a>> {
        self.current.as_ref().map(|token| &token.kind)
    }
}
