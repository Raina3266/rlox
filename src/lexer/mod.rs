use std::{collections::HashMap, sync::LazyLock};

use crate::error::error;

#[derive(Debug, Clone)]
pub struct Token {
    pub token_type: TokenType,
    pub lexeme: String,
    pub literal: Option<Literal>,
    pub line: usize,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TokenType {
    // Single-char
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Comma,
    Dot,
    Minus,
    Plus,
    Semicolon,
    Slash,
    Star,

    // One or Two char
    Bang,
    BangEqual,
    Equal,
    EqualEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,

    // Literals
    Identifier,
    String,
    Number,

    // Keywords
    And,
    Class,
    Else,
    True,
    False,
    Fun,
    For,
    If,
    Nil,
    Or,
    Print,
    Return,
    Super,
    This,
    Var,
    While,
    Eof,
}

#[derive(Debug, Clone)]
pub enum Literal {
    Bool(bool),
    Number(f64),
    String(String),
}

pub struct Scanner {
    source: Vec<char>,
    tokens: Vec<Token>,
    start: usize,
    current: usize,
    line: usize,
}

impl Scanner {
    pub fn new(source: &str) -> Self {
        let tokens = vec![];
        Scanner {
            source: source.chars().collect(),
            tokens,
            start: 0,
            current: 0,
            line: 1,
        }
    }

    pub fn scan_tokens(&mut self) -> Vec<Token> {
        while !self.is_at_end() {
            self.start = self.current;
            self.scan_token();
        }
        let end_of_file = Token {
            token_type: TokenType::Eof,
            lexeme: "".to_string(),
            literal: None,
            line: self.line,
        };
        self.tokens.push(end_of_file);
        self.tokens.clone()
    }

    pub fn scan_token(&mut self) {
        match self.advance() {
            '(' => self.add_token(TokenType::LeftParen),
            ')' => self.add_token(TokenType::RightParen),
            '{' => self.add_token(TokenType::LeftBrace),
            '}' => self.add_token(TokenType::RightBrace),
            ',' => self.add_token(TokenType::Comma),
            '.' => self.add_token(TokenType::Dot),
            '-' => self.add_token(TokenType::Minus),
            '+' => self.add_token(TokenType::Plus),
            ';' => self.add_token(TokenType::Semicolon),
            '*' => self.add_token(TokenType::Star),
            '!' => {
                if self.advance_if_equal('=') {
                    self.add_token(TokenType::BangEqual)
                }
                self.add_token(TokenType::Bang);
            }
            '=' => {
                if self.advance_if_equal('=') {
                    self.add_token(TokenType::EqualEqual)
                }
                self.add_token(TokenType::Equal);
            }
            '<' => {
                if self.advance_if_equal('=') {
                    self.add_token(TokenType::LessEqual)
                }
                self.add_token(TokenType::Less);
            }
            '>' => {
                if self.advance_if_equal('=') {
                    self.add_token(TokenType::GreaterEqual)
                }
                self.add_token(TokenType::Greater);
            }
            '/' => {
                if self.advance_if_equal('/') {
                    while self.peek() != Some('\n') && !self.is_at_end() {
                        self.advance();
                    }
                } else {
                    self.add_token(TokenType::Slash);
                }
            }
            ' ' | '\r' | '\t' => {}
            '\n' => {
                self.line += 1;
            }
            '"' => self.string_literal(),
            '0'..='9' => self.number(),
            c => {
                if c.is_ascii() {
                    self.identifier();
                } else {
                    error(self.line, "Unexpected character.".to_string())
                }
            }
        };
    }

    fn add_token(&mut self, token_type: TokenType) {
        self.add_token_with_literal(token_type, None);
    }

    fn add_token_with_literal(&mut self, token_type: TokenType, literal: Option<Literal>) {
        let chars = &self.source[self.start..self.current];
        self.tokens.push(Token {
            token_type,
            lexeme: chars.iter().collect(),
            literal,
            line: self.line,
        });
    }

    fn is_at_end(&self) -> bool {
        self.current > self.source.len()
    }

    fn advance(&mut self) -> char {
        let c = self.source[self.current];
        self.current += 1;
        c
    }

    fn advance_if_equal(&mut self, expected: char) -> bool {
        if self.is_at_end() {
            return false;
        }
        if self.source[self.current] != expected {
            return false;
        }

        self.current += 1;
        true
    }

    fn peek(&self) -> Option<char> {
        if self.is_at_end() {
            return None;
        }
        Some(self.source[self.current])
    }

    fn peek_next(&self) -> Option<char> {
        if self.current + 1 >= self.source.len() {
            return None;
        }
        Some(self.source[self.current + 1])
    }

    fn string_literal(&mut self) {
        while self.peek() != Some('"') && !self.is_at_end() {
            // allow multilines in string literal
            if self.peek() == Some('\n') {
                self.line += 1;
            }
            self.advance();
        }
        if self.is_at_end() {
            error(self.line, "Unterminated string.".to_string());
            return;
        }
        let value: String = self.source[self.start + 1..self.current].iter().collect();
        self.add_token_with_literal(TokenType::String, Some(Literal::String(value)));
        self.advance();
    }

    fn number(&mut self) {
        while self.peek().is_some_and(|c| c.is_ascii_digit()) {
            self.advance();
        }

        if self.peek().is_some_and(|c| c == '.')
            && self.peek_next().is_some_and(|c| c.is_ascii_digit())
        {
            self.advance();
            while self.peek().is_some_and(|c| c.is_ascii_digit()) {
                self.advance();
            }
        }
        let value: String = self.source[self.start..self.current].iter().collect();
        let Ok(number) = value.parse::<f64>() else {
            error(self.line, "Unterminated number.".to_string());
            return;
        };
        self.add_token_with_literal(TokenType::Number, Some(Literal::Number(number)));
        self.advance();
    }
    
    fn identifier(&mut self) {
        while self
            .peek()
            .is_some_and(|c| c.is_ascii() && c.is_ascii_digit())
        {
            self.advance();
        }

        let value: String = self.source[self.start..self.current].iter().collect();
        if let Some(keyword) = KEYWORD_MAP.get(value.as_str()) {
            self.add_token_with_literal(*keyword, None);
        } else {
            self.add_token_with_literal(TokenType::Identifier, None);
        }
    }
}

// data you only need to build once, and can access it whenever, no runtime input needed
static KEYWORD_MAP: LazyLock<HashMap<&'static str, TokenType>> = LazyLock::new(|| {
    let mut map = HashMap::new();

    map.insert("and", TokenType::And);
    map.insert("class", TokenType::Class);
    map.insert("else", TokenType::Else);
    map.insert("false", TokenType::False);
    map.insert("for", TokenType::For);
    map.insert("fun", TokenType::Fun);
    map.insert("if", TokenType::If);
    map.insert("nil", TokenType::Nil);
    map.insert("or", TokenType::Or);
    map.insert("print", TokenType::Print);
    map.insert("return", TokenType::Return);
    map.insert("super", TokenType::Super);
    map.insert("this", TokenType::This);
    map.insert("true", TokenType::True);
    map.insert("var", TokenType::Var);
    map.insert("while", TokenType::While);
    map
});
