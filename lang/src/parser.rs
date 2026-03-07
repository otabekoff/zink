// src/parser.rs — Zink Language Parser
// Recursive-descent parser producing an AST

#![allow(dead_code)]
use crate::lexer::{Token, TokenKind};

// ── AST Node Types ──────────────────────────────────────────────
#[derive(Debug, Clone)]
pub enum Stmt {
    VarDecl {
        name: String,
        value: Expr,
        line: usize,
    },
    Assign {
        target: Expr,
        value: Expr,
    },
    FnDecl {
        name: String,
        params: Vec<String>,
        body: Vec<Stmt>,
        line: usize,
    },
    Return {
        value: Option<Expr>,
        line: usize,
    },
    If {
        cond: Expr,
        then: Vec<Stmt>,
        else_: Option<Box<Stmt>>,
        line: usize,
    },
    While {
        cond: Expr,
        body: Vec<Stmt>,
        line: usize,
    },
    Loop {
        count: Expr,
        body: Vec<Stmt>,
        line: usize,
    },
    Say {
        value: Expr,
        line: usize,
    },
    Expr {
        expr: Expr,
    },
    Block {
        body: Vec<Stmt>,
    },
}

#[derive(Debug, Clone)]
pub enum Expr {
    Number(f64),
    Str(String),
    Bool(bool),
    Nil,
    Ident(String),
    Array(Vec<Expr>),
    BinOp {
        op: BinOp,
        left: Box<Expr>,
        right: Box<Expr>,
    },
    UnaryOp {
        op: UnaryOp,
        expr: Box<Expr>,
    },
    Call {
        callee: Box<Expr>,
        args: Vec<Expr>,
    },
    Index {
        obj: Box<Expr>,
        idx: Box<Expr>,
    },
    Prop {
        obj: Box<Expr>,
        prop: String,
    },
    Lambda {
        params: Vec<String>,
        body: Vec<Stmt>,
    }, // ← add this line
}

#[derive(Debug, Clone, PartialEq)]
pub enum BinOp {
    Add,
    Sub,
    Mul,
    Div,
    Mod,
    Eq,
    Neq,
    Lt,
    Gt,
    Lte,
    Gte,
    And,
    Or,
}

#[derive(Debug, Clone, PartialEq)]
pub enum UnaryOp {
    Neg,
    Not,
}

// ── Parse Error ─────────────────────────────────────────────────

#[derive(Debug)]
pub struct ParseError {
    pub message: String,
    pub line: usize,
}

macro_rules! perr {
    ($line:expr, $($arg:tt)*) => {
        ParseError { message: format!($($arg)*), line: $line }
    };
}

// ── Parser ──────────────────────────────────────────────────────

pub struct Parser {
    tokens: Vec<Token>,
    pos: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self { tokens, pos: 0 }
    }

    fn peek(&self) -> &Token {
        &self.tokens[self.pos]
    }
    fn line(&self) -> usize {
        self.peek().line
    }

    fn eat(&mut self) -> &Token {
        let t = &self.tokens[self.pos];
        if self.pos + 1 < self.tokens.len() {
            self.pos += 1;
        }
        t
    }

    fn check(&self, kind: &TokenKind) -> bool {
        std::mem::discriminant(&self.peek().kind) == std::mem::discriminant(kind)
    }

    fn check_exact(&self, kind: &TokenKind) -> bool {
        &self.peek().kind == kind
    }

    fn match_tok(&mut self, kind: &TokenKind) -> bool {
        if self.check_exact(kind) {
            self.eat();
            true
        } else {
            false
        }
    }

    fn expect(&mut self, kind: &TokenKind) -> Result<&Token, ParseError> {
        if self.check_exact(kind) {
            Ok(self.eat())
        } else {
            Err(perr!(
                self.line(),
                "Expected {:?}, got {:?}",
                kind,
                self.peek().kind
            ))
        }
    }

    fn is_eof(&self) -> bool {
        self.peek().kind == TokenKind::Eof
    }

    // ── Public entry point ──
    pub fn parse(&mut self) -> Result<Vec<Stmt>, ParseError> {
        let mut stmts = Vec::new();
        while !self.is_eof() {
            stmts.push(self.parse_stmt()?);
        }
        Ok(stmts)
    }

    fn parse_block(&mut self) -> Result<Vec<Stmt>, ParseError> {
        self.expect(&TokenKind::LBrace)?;
        let mut stmts = Vec::new();
        while !self.check_exact(&TokenKind::RBrace) && !self.is_eof() {
            stmts.push(self.parse_stmt()?);
        }
        self.expect(&TokenKind::RBrace)?;
        Ok(stmts)
    }

    fn parse_stmt(&mut self) -> Result<Stmt, ParseError> {
        match &self.peek().kind.clone() {
            TokenKind::Let => self.parse_var_decl(),
            TokenKind::Fn => self.parse_fn_decl(),
            TokenKind::Return => self.parse_return(),
            TokenKind::If => self.parse_if(),
            TokenKind::While => self.parse_while(),
            TokenKind::Loop => self.parse_loop(),
            TokenKind::Say => self.parse_say(),
            _ => self.parse_expr_stmt(),
        }
    }

    fn parse_var_decl(&mut self) -> Result<Stmt, ParseError> {
        let line = self.line();
        self.eat(); // let
        let name = match self.eat().kind.clone() {
            TokenKind::Ident(n) => n,
            _ => return Err(perr!(line, "Expected identifier after 'let'")),
        };
        self.expect(&TokenKind::Assign)?;
        let value = self.parse_expr()?;
        Ok(Stmt::VarDecl { name, value, line })
    }

    fn parse_fn_decl(&mut self) -> Result<Stmt, ParseError> {
        let line = self.line();
        self.eat(); // fn
        let name = match self.eat().kind.clone() {
            TokenKind::Ident(n) => n,
            _ => return Err(perr!(line, "Expected function name")),
        };
        self.expect(&TokenKind::LParen)?;
        let mut params = Vec::new();
        while !self.check_exact(&TokenKind::RParen) && !self.is_eof() {
            match self.eat().kind.clone() {
                TokenKind::Ident(p) => params.push(p),
                _ => return Err(perr!(self.line(), "Expected parameter name")),
            }
            if !self.match_tok(&TokenKind::Comma) {
                break;
            }
        }
        self.expect(&TokenKind::RParen)?;
        let body = self.parse_block()?;
        Ok(Stmt::FnDecl {
            name,
            params,
            body,
            line,
        })
    }

    fn parse_return(&mut self) -> Result<Stmt, ParseError> {
        let line = self.line();
        self.eat(); // return
        let value = if !self.check_exact(&TokenKind::RBrace) && !self.is_eof() {
            Some(self.parse_expr()?)
        } else {
            None
        };
        Ok(Stmt::Return { value, line })
    }

    fn parse_if(&mut self) -> Result<Stmt, ParseError> {
        let line = self.line();
        self.eat(); // if
        let cond = self.parse_expr()?;
        let then = self.parse_block()?;
        let else_ = if self.match_tok(&TokenKind::Else) {
            if self.check_exact(&TokenKind::If) {
                Some(Box::new(self.parse_if()?))
            } else {
                Some(Box::new(Stmt::Block {
                    body: self.parse_block()?,
                }))
            }
        } else {
            None
        };
        Ok(Stmt::If {
            cond,
            then,
            else_,
            line,
        })
    }

    fn parse_while(&mut self) -> Result<Stmt, ParseError> {
        let line = self.line();
        self.eat(); // while
        let cond = self.parse_expr()?;
        let body = self.parse_block()?;
        Ok(Stmt::While { cond, body, line })
    }

    fn parse_loop(&mut self) -> Result<Stmt, ParseError> {
        let line = self.line();
        self.eat(); // loop
        let count = self.parse_expr()?;
        self.expect(&TokenKind::Times)?;
        let body = self.parse_block()?;
        Ok(Stmt::Loop { count, body, line })
    }

    fn parse_say(&mut self) -> Result<Stmt, ParseError> {
        let line = self.line();
        self.eat(); // say
        Ok(Stmt::Say {
            value: self.parse_expr()?,
            line,
        })
    }

    fn parse_expr_stmt(&mut self) -> Result<Stmt, ParseError> {
        let expr = self.parse_expr()?;
        if self.match_tok(&TokenKind::Assign) {
            let value = self.parse_expr()?;
            Ok(Stmt::Assign {
                target: expr,
                value,
            })
        } else {
            Ok(Stmt::Expr { expr })
        }
    }

    // ── Expressions (Pratt precedence climbing) ──

    fn parse_expr(&mut self) -> Result<Expr, ParseError> {
        self.parse_or()
    }

    fn parse_or(&mut self) -> Result<Expr, ParseError> {
        let mut l = self.parse_and()?;
        while self.match_tok(&TokenKind::Or) {
            let r = self.parse_and()?;
            l = Expr::BinOp {
                op: BinOp::Or,
                left: Box::new(l),
                right: Box::new(r),
            };
        }
        Ok(l)
    }

    fn parse_and(&mut self) -> Result<Expr, ParseError> {
        let mut l = self.parse_equality()?;
        while self.match_tok(&TokenKind::And) {
            let r = self.parse_equality()?;
            l = Expr::BinOp {
                op: BinOp::And,
                left: Box::new(l),
                right: Box::new(r),
            };
        }
        Ok(l)
    }

    fn parse_equality(&mut self) -> Result<Expr, ParseError> {
        let mut l = self.parse_comparison()?;
        loop {
            let op = if self.match_tok(&TokenKind::Eq) {
                BinOp::Eq
            } else if self.match_tok(&TokenKind::Neq) {
                BinOp::Neq
            } else {
                break;
            };
            let r = self.parse_comparison()?;
            l = Expr::BinOp {
                op,
                left: Box::new(l),
                right: Box::new(r),
            };
        }
        Ok(l)
    }

    fn parse_comparison(&mut self) -> Result<Expr, ParseError> {
        let mut l = self.parse_add()?;
        loop {
            let op = if self.match_tok(&TokenKind::Lt) {
                BinOp::Lt
            } else if self.match_tok(&TokenKind::Gt) {
                BinOp::Gt
            } else if self.match_tok(&TokenKind::Lte) {
                BinOp::Lte
            } else if self.match_tok(&TokenKind::Gte) {
                BinOp::Gte
            } else {
                break;
            };
            let r = self.parse_add()?;
            l = Expr::BinOp {
                op,
                left: Box::new(l),
                right: Box::new(r),
            };
        }
        Ok(l)
    }

    fn parse_add(&mut self) -> Result<Expr, ParseError> {
        let mut l = self.parse_mul()?;
        loop {
            let op = if self.match_tok(&TokenKind::Plus) {
                BinOp::Add
            } else if self.match_tok(&TokenKind::Minus) {
                BinOp::Sub
            } else {
                break;
            };
            let r = self.parse_mul()?;
            l = Expr::BinOp {
                op,
                left: Box::new(l),
                right: Box::new(r),
            };
        }
        Ok(l)
    }

    fn parse_mul(&mut self) -> Result<Expr, ParseError> {
        let mut l = self.parse_unary()?;
        loop {
            let op = if self.match_tok(&TokenKind::Star) {
                BinOp::Mul
            } else if self.match_tok(&TokenKind::Slash) {
                BinOp::Div
            } else if self.match_tok(&TokenKind::Percent) {
                BinOp::Mod
            } else {
                break;
            };
            let r = self.parse_unary()?;
            l = Expr::BinOp {
                op,
                left: Box::new(l),
                right: Box::new(r),
            };
        }
        Ok(l)
    }

    fn parse_unary(&mut self) -> Result<Expr, ParseError> {
        if self.match_tok(&TokenKind::Minus) {
            return Ok(Expr::UnaryOp {
                op: UnaryOp::Neg,
                expr: Box::new(self.parse_unary()?),
            });
        }
        if self.match_tok(&TokenKind::Not) {
            return Ok(Expr::UnaryOp {
                op: UnaryOp::Not,
                expr: Box::new(self.parse_unary()?),
            });
        }
        self.parse_postfix()
    }

    fn parse_postfix(&mut self) -> Result<Expr, ParseError> {
        let mut expr = self.parse_primary()?;
        loop {
            if self.check_exact(&TokenKind::LParen) {
                self.eat();
                let mut args = Vec::new();
                while !self.check_exact(&TokenKind::RParen) && !self.is_eof() {
                    args.push(self.parse_expr()?);
                    if !self.match_tok(&TokenKind::Comma) {
                        break;
                    }
                }
                self.expect(&TokenKind::RParen)?;
                expr = Expr::Call {
                    callee: Box::new(expr),
                    args,
                };
            } else if self.check_exact(&TokenKind::LBracket) {
                self.eat();
                let idx = self.parse_expr()?;
                self.expect(&TokenKind::RBracket)?;
                expr = Expr::Index {
                    obj: Box::new(expr),
                    idx: Box::new(idx),
                };
            } else if self.check_exact(&TokenKind::Dot) {
                self.eat();
                let prop = match self.eat().kind.clone() {
                    TokenKind::Ident(n) => n,
                    _ => return Err(perr!(self.line(), "Expected property name after '.'")),
                };
                expr = Expr::Prop {
                    obj: Box::new(expr),
                    prop,
                };
            } else {
                break;
            }
        }
        Ok(expr)
    }

    fn parse_primary(&mut self) -> Result<Expr, ParseError> {
        let line = self.line();
        match self.peek().kind.clone() {
            TokenKind::Number(n) => {
                self.eat();
                Ok(Expr::Number(n))
            }
            TokenKind::Str(s) => {
                self.eat();
                Ok(Expr::Str(s))
            }
            TokenKind::Bool(b) => {
                self.eat();
                Ok(Expr::Bool(b))
            }
            TokenKind::Nil => {
                self.eat();
                Ok(Expr::Nil)
            }
            TokenKind::Ident(name) => {
                self.eat();
                Ok(Expr::Ident(name))
            }
            TokenKind::LBracket => {
                self.eat();
                let mut items = Vec::new();
                while !self.check_exact(&TokenKind::RBracket) && !self.is_eof() {
                    items.push(self.parse_expr()?);
                    if !self.match_tok(&TokenKind::Comma) {
                        break;
                    }
                }
                self.expect(&TokenKind::RBracket)?;
                Ok(Expr::Array(items))
            }
            TokenKind::LParen => {
                self.eat();
                let expr = self.parse_expr()?;
                self.expect(&TokenKind::RParen)?;
                Ok(expr)
            }
            TokenKind::Fn => {
                self.eat(); // consume fn
                self.expect(&TokenKind::LParen)?;
                let mut params = Vec::new();
                while !self.check_exact(&TokenKind::RParen) && !self.is_eof() {
                    match self.eat().kind.clone() {
                        TokenKind::Ident(p) => params.push(p),
                        _ => return Err(perr!(self.line(), "Expected parameter name in lambda")),
                    }
                    if !self.match_tok(&TokenKind::Comma) {
                        break;
                    }
                }
                self.expect(&TokenKind::RParen)?;
                let body = self.parse_block()?;
                Ok(Expr::Lambda { params, body })
            }
            other => Err(perr!(line, "Unexpected token: {:?}", other)),
        }
    }

    // Public wrapper used by the interpreter for string interpolation
    pub fn _expr_pub(&mut self) -> Result<Expr, ParseError> {
        self.parse_expr()
    }
}
