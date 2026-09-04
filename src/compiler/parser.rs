use crate::compiler::lexer::Lexer;
use crate::compiler::lexer::token::{Token, TokenKind};
use crate::compiler::parser::ast::{
    BlockExpr, CallExpr, Decl, Expr, ExternDecl, File, FuncDecl, FuncSig, Ident, Int, Param, Stmt,
};

pub mod ast;

pub struct Parser<'a> {
    src: &'a str,
    lexer: Lexer<'a>,
}

impl<'a> Parser<'a> {
    pub fn new(src: &'a str) -> Self {
        Self {
            src,
            lexer: Lexer::new(src),
        }
    }

    pub fn parse(&mut self) -> File {
        let mut decls = Vec::new();

        self.skip_newlines();

        while self.lexer.peek_kind() != &TokenKind::Eof {
            decls.push(self.parse_decl());
            self.skip_newlines();
        }

        File { decls }
    }

    fn parse_decl(&mut self) -> Decl {
        match self.lexer.peek_kind() {
            TokenKind::Extern => Decl::Extern(self.parse_extern_decl()),
            TokenKind::Func => Decl::Func(self.parse_func_decl()),
            kind => panic!("Expected Decl, found {:?}", kind),
        }
    }

    fn parse_extern_decl(&mut self) -> ExternDecl {
        let external = self.expect(TokenKind::Extern);
        self.expect(TokenKind::OpenBrace);
        self.skip_newlines();

        let mut funcs = Vec::new();

        while self.lexer.peek_kind() != &TokenKind::CloseBrace {
            funcs.push(self.parse_func_decl());
            self.skip_newlines();
        }

        let close_brace = self.expect(TokenKind::CloseBrace);
        let span = external.span.join(close_brace.span);

        ExternDecl { funcs, span }
    }

    fn parse_func_decl(&mut self) -> FuncDecl {
        let func = self.expect(TokenKind::Func);
        let name = self.parse_ident();
        let sig = self.parse_func_sig();

        let (body, span) = if self.lexer.peek_kind() == &TokenKind::OpenBrace {
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
        match self.lexer.peek_kind() {
            TokenKind::OpenBrace => Expr::Block(self.parse_block_expr()),
            TokenKind::Ident => {
                let ident = self.parse_ident();

                if self.lexer.peek_kind() == &TokenKind::OpenParen {
                    Expr::Call(self.parse_call_expr(ident))
                } else {
                    Expr::Ident(ident)
                }
            }
            TokenKind::Int => Expr::Int(self.parse_int()),
            kind => panic!("Expected Expr, found {:?}", kind),
        }
    }

    fn parse_block_expr(&mut self) -> BlockExpr {
        let open_brace = self.expect(TokenKind::OpenBrace);
        self.skip_newlines();

        let mut stmts = Vec::new();

        while self.lexer.peek_kind() != &TokenKind::CloseBrace {
            stmts.push(Stmt::Expr(self.parse_expr()));
            self.skip_newlines();
        }

        let close_brace = self.expect(TokenKind::CloseBrace);
        let span = open_brace.span.join(close_brace.span);

        BlockExpr { stmts, span }
    }

    fn parse_call_expr(&mut self, name: Ident) -> CallExpr {
        self.expect(TokenKind::OpenParen);
        self.skip_newlines();

        let mut args = Vec::new();

        while self.lexer.peek_kind() != &TokenKind::CloseParen {
            args.push(self.parse_expr());

            if self.lexer.peek_kind() == &TokenKind::Comma {
                self.lexer.next_token();
                self.skip_newlines();
            } else {
                break;
            }
        }

        self.skip_newlines();
        let close_paren = self.expect(TokenKind::CloseParen);
        let span = name.span.join(close_paren.span);

        CallExpr { name, args, span }
    }

    fn parse_func_sig(&mut self) -> FuncSig {
        let open_paren = self.expect(TokenKind::OpenParen);
        self.skip_newlines();

        let mut params = Vec::new();

        while self.lexer.peek_kind() != &TokenKind::CloseParen {
            params.push(self.parse_param());

            if self.lexer.peek_kind() == &TokenKind::Comma {
                self.lexer.next_token();
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
        let ident = self.expect(TokenKind::Ident);

        Ident {
            value: self.text(&ident).to_string(),
            span: ident.span,
        }
    }

    fn parse_int(&mut self) -> Int {
        let int = self.expect(TokenKind::Int);

        Int::Unsigned(self.text(&int).parse::<u64>().unwrap())
    }

    fn expect(&mut self, expected: TokenKind) -> Token {
        let token = self.lexer.next_token();

        if token.kind != expected {
            panic!("Expected {:?}, found {:?}", expected, token.kind);
        }

        token
    }

    fn skip_newlines(&mut self) {
        self.skip_while(|kind| kind == &TokenKind::Newline);
    }

    fn skip_while(&mut self, predicate: impl Fn(&TokenKind) -> bool + Copy) {
        while self.skip_if(predicate) {}
    }

    fn skip_if(&mut self, predicate: impl Fn(&TokenKind) -> bool) -> bool {
        if predicate(self.lexer.peek_kind()) {
            self.lexer.next_token();
            true
        } else {
            false
        }
    }

    fn text(&self, token: &Token) -> &str {
        &self.src[token.span.start..token.span.end]
    }
}
