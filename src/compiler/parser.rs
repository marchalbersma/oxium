use crate::compiler::ast::{
    BlockExpr, CallExpr, Decl, Expr, ExternDecl, File, FuncDecl, FuncSig, Ident, Int, Lit, Param,
};
use crate::compiler::lexer::Lexer;
use crate::compiler::span::Span;
use crate::compiler::token;
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
            Some(TokenKind::Extern) => Decl::Extern(self.parse_extern_decl()),
            Some(TokenKind::Func) => Decl::Func(self.parse_func_decl()),

            kind => panic!("Expected Decl, found {:?}", kind),
        }
    }

    fn parse_extern_decl(&mut self) -> ExternDecl {
        let ext = self.expect(TokenKind::Extern);
        self.expect(TokenKind::OpenBrace);

        let mut funcs = Vec::new();

        self.skip_newlines();

        while self.kind() != Some(&TokenKind::CloseBrace) {
            funcs.push(self.parse_func_decl());

            self.skip_newlines();
        }

        let close_brace = self.expect(TokenKind::CloseBrace);

        let span = ext.span.join(close_brace.span);

        ExternDecl { funcs, span }
    }

    fn parse_func_decl(&mut self) -> FuncDecl {
        let func = self.expect(TokenKind::Func);
        let name = self.parse_ident();
        let sig = self.parse_func_sig();

        let (body, span) = if self.kind() == Some(&TokenKind::OpenBrace) {
            let body = self.parse_block_expr();
            let span = func.span.join(body.span);

            (Some(body), span)
        } else {
            (None, func.span.join(sig.span))
        };

        FuncDecl {
            name,
            sig,
            body,
            span,
        }
    }

    fn parse_expr(&mut self) -> Expr {
        match self.kind() {
            Some(TokenKind::OpenBrace) => Expr::Block(self.parse_block_expr()),
            Some(TokenKind::Ident(_)) => self.parse_ident_or_call_expr(),
            _ => Expr::Lit(self.parse_lit()),
        }
    }

    fn parse_block_expr(&mut self) -> BlockExpr {
        let open_brace = self.expect(TokenKind::OpenBrace);

        let mut expressions = Vec::new();

        self.skip_newlines();

        while self.kind() != Some(&TokenKind::CloseBrace) {
            expressions.push(self.parse_expr());

            self.skip_newlines();
        }

        let close_brace = self.expect(TokenKind::CloseBrace);

        let span = open_brace.span.join(close_brace.span);

        BlockExpr { expressions, span }
    }

    fn parse_ident_or_call_expr(&mut self) -> Expr {
        let name = self.parse_ident();

        if self.kind() == Some(&TokenKind::OpenParen) {
            self.consume();

            let mut args = Vec::new();

            self.skip_newlines();

            while self.kind() != Some(&TokenKind::CloseParen) {
                args.push(self.parse_expr());

                if self.kind() == Some(&TokenKind::Comma) {
                    self.consume();
                    self.skip_newlines();
                } else {
                    break;
                }
            }

            self.skip_newlines();

            let close_paren = self.expect(TokenKind::CloseParen);

            let span = name.span.join(close_paren.span);

            Expr::Call(CallExpr { name, args, span })
        } else {
            Expr::Ident(Ident {
                value: name.value,
                span: name.span,
            })
        }
    }

    fn parse_func_sig(&mut self) -> FuncSig {
        let open_paren = self.expect(TokenKind::OpenParen);

        let mut params = Vec::new();

        self.skip_newlines();

        while self.kind() != Some(&TokenKind::CloseParen) {
            params.push(self.parse_param());

            if self.kind() == Some(&TokenKind::Comma) {
                self.consume();
                self.skip_newlines();
            } else {
                break;
            }
        }

        self.skip_newlines();

        let close_paren = self.expect(TokenKind::CloseParen);

        let span = open_paren.span.join(close_paren.span);

        FuncSig { params, span }
    }

    fn parse_param(&mut self) -> Param {
        let name = self.parse_ident();
        self.expect(TokenKind::Colon);
        let ty = self.parse_ident();

        let span = name.span.join(ty.span);

        Param { name, ty, span }
    }

    fn parse_ident(&mut self) -> Ident {
        let (value, span) = self.expect_ident();

        Ident { value, span }
    }

    fn parse_lit(&mut self) -> Lit {
        let lit = match self.kind() {
            Some(TokenKind::Int(int)) => match int {
                token::Int::Signed(val) => Lit::Int(Int::Signed(*val)),
                token::Int::Unsigned(val) => Lit::Int(Int::Unsigned(*val)),
            },

            kind => panic!("Expected Int, found {:?}", kind),
        };

        self.consume();

        lit
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
