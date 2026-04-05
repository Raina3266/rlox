use std::str::Chars;

pub struct Token {
    token_type: TokenType,
    lexeme: String,
    literal: Option<Literal>,
    line: usize,
}

enum TokenType {
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
    // Bang,
    // BangEqual,
    // Equal,
    // EqualEqual,
    // Greater,
    // GreaterEqual,
    // Less,
    // LessEqual,

    // Keywords
    // And,
    // Class,
    // Else,
    // True,
    // False,
    // Fun,
    // For,
    // If,
    // Nil,
    // Or,
    // Print,
    // Return,
    // Super,
    // This,
    // Var,
    // While,

    Eof,
}



enum Literal {
    Bool(bool),
    Number(f64),
    String(String),
}

struct Scanner {
    source: Vec<char>,
    tokens: Vec<Token>,
    start: usize,
    current: usize,
    line: usize,
}

impl Scanner {
    fn new(source: &str) -> Self {
        let tokens = vec![];
        Scanner {
            source: source.chars().collect(),
            tokens,
            start: 0,
            current: 0,
            line: 1,
        }
    }

    fn scan_tokens(&mut self) {
        while !self.is_at_end() {
            self.start = self.current;
            self.scan_tokens();
        }
        let end_of_file = Token {
            token_type: TokenType::Eof,
            lexeme: "".to_string(),
            literal: None,
            line: self.line,
        };
        self.tokens.push(end_of_file);
    }

    fn scan_token(&mut self) {
        match self.advance() {
            '(' => self.add_token(TokenType::LeftParen),
            ')' => self.add_token(TokenType::RightParen),
            '{' => self.add_token(TokenType::LeftBrace),
            '}' => self.add_token(TokenType::RightBrace),
            ',' => self.add_token(TokenType::Comma),
            '.' => self.add_token(TokenType::Dot),
            '-' => self.add_token(TokenType::Slash),
            '+' => self.add_token(TokenType::Plus),
            ';' => self.add_token(TokenType::Semicolon),
            '*' => self.add_token(TokenType::Star),
            _ => {},
        };
    }

    fn advance(&mut self) -> char {
        self.current += 1;
        return self.source[self.current - 1];
    }

    fn add_token(&mut self, token_type: TokenType) {
        self.add_token_with_literal(token_type, None);
    }

    fn add_token_with_literal(&mut self, token_type: TokenType, literal: Option<Literal>) {
        let chars = &self.source[self.start..self.current];
        self.tokens.push(Token { token_type, lexeme: chars.iter().collect(), literal, line: self.line });
    }

    fn is_at_end(&self) -> bool {
        self.current > self.source.len()
    }
}
