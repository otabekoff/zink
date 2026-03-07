// src/lexer.rs — Zink Language Lexer
// Tokenizes source text into a flat Vec<Token>

// #![allow(dead_code)]

#[derive(Debug, Clone, PartialEq)]
pub enum TokenKind {
    // Literals
    Number(f64),
    Str(String),
    Bool(bool),
    Nil,
    // Identifiers & Keywords
    Ident(String),
    Let,
    Fn,
    Return,
    If,
    Else,
    While,
    Loop,
    Times,
    Say,
    And,
    Or,
    Not,
    // Operators
    Plus,
    Minus,
    Star,
    Slash,
    Percent,
    Eq,
    Neq,
    Lt,
    Gt,
    Lte,
    Gte,
    Assign,
    // Delimiters
    LParen,
    RParen,
    LBrace,
    RBrace,
    LBracket,
    RBracket,
    Comma,
    Dot,
    // Semicolon,
    // Special
    Eof,
}

#[derive(Debug, Clone)]
pub struct Token {
    pub kind: TokenKind,
    pub line: usize,
}

impl Token {
    fn new(kind: TokenKind, line: usize) -> Self {
        Self { kind, line }
    }
}

#[derive(Debug)]
pub struct LexError {
    pub message: String,
    pub line: usize,
}

pub struct Lexer {
    src: Vec<char>,
    pos: usize,
    line: usize,
}

impl Lexer {
    pub fn new(src: &str) -> Self {
        Self {
            src: src.chars().collect(),
            pos: 0,
            line: 1,
        }
    }

    fn peek(&self, offset: usize) -> Option<char> {
        self.src.get(self.pos + offset).copied()
    }

    fn eat(&mut self) -> Option<char> {
        let c = self.src.get(self.pos).copied()?;
        self.pos += 1;
        if c == '\n' {
            self.line += 1;
        }
        Some(c)
    }

    fn skip_whitespace(&mut self) {
        while matches!(self.peek(0), Some(' ' | '\t' | '\r' | '\n')) {
            self.eat();
        }
    }

    fn skip_comment(&mut self) {
        while !matches!(self.peek(0), Some('\n') | None) {
            self.eat();
        }
    }

    pub fn tokenize(&mut self) -> Result<Vec<Token>, LexError> {
        let mut tokens = Vec::new();

        loop {
            self.skip_whitespace();
            let line = self.line;
            let c = match self.peek(0) {
                Some(c) => c,
                None => {
                    tokens.push(Token::new(TokenKind::Eof, line));
                    break;
                }
            };

            if c == '#' {
                self.skip_comment();
                continue;
            }

            if c == '"' {
                tokens.push(self.read_string()?);
                continue;
            }

            if c.is_ascii_digit() {
                tokens.push(self.read_number());
                continue;
            }

            if c.is_alphabetic() || c == '_' {
                tokens.push(self.read_ident());
                continue;
            }

            self.eat();
            let kind = match c {
                '+' => TokenKind::Plus,
                '-' => TokenKind::Minus,
                '*' => TokenKind::Star,
                '%' => TokenKind::Percent,
                '(' => TokenKind::LParen,
                ')' => TokenKind::RParen,
                '{' => TokenKind::LBrace,
                '}' => TokenKind::RBrace,
                '[' => TokenKind::LBracket,
                ']' => TokenKind::RBracket,
                ',' => TokenKind::Comma,
                '.' => TokenKind::Dot,
                ';' => continue, // optional semicolons are ignored
                '/' => {
                    if self.peek(0) == Some('/') {
                        self.skip_comment();
                        continue;
                    }
                    TokenKind::Slash
                }
                '=' => {
                    if self.peek(0) == Some('=') {
                        self.eat();
                        TokenKind::Eq
                    } else {
                        TokenKind::Assign
                    }
                }
                '!' => {
                    if self.peek(0) == Some('=') {
                        self.eat();
                        TokenKind::Neq
                    } else {
                        TokenKind::Not
                    }
                }
                '<' => {
                    if self.peek(0) == Some('=') {
                        self.eat();
                        TokenKind::Lte
                    } else {
                        TokenKind::Lt
                    }
                }
                '>' => {
                    if self.peek(0) == Some('=') {
                        self.eat();
                        TokenKind::Gte
                    } else {
                        TokenKind::Gt
                    }
                }
                '&' if self.peek(0) == Some('&') => {
                    self.eat();
                    TokenKind::And
                }
                '|' if self.peek(0) == Some('|') => {
                    self.eat();
                    TokenKind::Or
                }
                other => {
                    return Err(LexError {
                        message: format!("Unknown character: '{other}'"),
                        line,
                    });
                }
            };
            tokens.push(Token::new(kind, line));
        }

        Ok(tokens)
    }

    fn read_string(&mut self) -> Result<Token, LexError> {
        let line = self.line;
        self.eat(); // consume opening "
        let mut s = String::new();
        loop {
            match self.peek(0) {
                None => {
                    return Err(LexError {
                        message: "Unterminated string literal".into(),
                        line,
                    });
                }
                Some('"') => {
                    self.eat();
                    break;
                }
                Some('\\') => {
                    self.eat();
                    match self.eat() {
                        Some('n') => s.push('\n'),
                        Some('t') => s.push('\t'),
                        Some('"') => s.push('"'),
                        Some('\\') => s.push('\\'),
                        Some(c) => s.push(c),
                        None => {
                            return Err(LexError {
                                message: "Unexpected EOF in string escape".into(),
                                line,
                            });
                        }
                    }
                }
                Some(c) => {
                    s.push(c);
                    self.eat();
                }
            }
        }
        Ok(Token::new(TokenKind::Str(s), line))
    }

    fn read_number(&mut self) -> Token {
        let line = self.line;
        let mut num = String::new();
        while matches!(self.peek(0), Some(c) if c.is_ascii_digit() || c == '.') {
            num.push(self.eat().unwrap());
        }
        let value: f64 = num.parse().unwrap_or(0.0);
        Token::new(TokenKind::Number(value), line)
    }

    fn read_ident(&mut self) -> Token {
        let line = self.line;
        let mut ident = String::new();
        while matches!(self.peek(0), Some(c) if c.is_alphanumeric() || c == '_') {
            ident.push(self.eat().unwrap());
        }
        let kind = match ident.as_str() {
            "let" => TokenKind::Let,
            "fn" => TokenKind::Fn,
            "return" => TokenKind::Return,
            "if" => TokenKind::If,
            "else" => TokenKind::Else,
            "while" => TokenKind::While,
            "loop" => TokenKind::Loop,
            "times" => TokenKind::Times,
            "say" => TokenKind::Say,
            "and" => TokenKind::And,
            "or" => TokenKind::Or,
            "not" => TokenKind::Not,
            "true" => TokenKind::Bool(true),
            "false" => TokenKind::Bool(false),
            "null" => TokenKind::Nil,
            _ => TokenKind::Ident(ident),
        };
        Token::new(kind, line)
    }
}
