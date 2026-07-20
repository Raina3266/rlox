use crate::{
    error::report,
    lexer::{self, Token, TokenType},
};

#[derive(Debug, Clone)]
enum Expr {
    Literal(ExprLiteral),
    Unary {
        operator: UnaryOp,
        expr: Box<Self>,
    },
    Binary {
        operator: BinaryOp,
        lhs: Box<Self>,
        rhs: Box<Self>,
    },
    Grouping(Box<Self>),
}

impl Expr {
    fn pretty_print(&self) -> String {
        match self {
            Expr::Literal(ExprLiteral::True) => {
                return String::from("true");
            }
            Expr::Literal(ExprLiteral::False) => {
                return String::from("false");
            }
            Expr::Literal(ExprLiteral::Nil) => {
                return String::from("nil");
            }
            Expr::Literal(ExprLiteral::Number(num)) => {
                return num.to_string();
            }
            Expr::Literal(ExprLiteral::String(str)) => {
                return str.clone();
            }
            Expr::Unary { operator, expr } => {
                let op_str = match operator {
                    UnaryOp::Minus => "-",
                    UnaryOp::Bang => "!",
                };
                let expr = vec![&**expr];
                return Expr::parenthesize(op_str.to_string(), expr);
            }
            Expr::Binary { operator, lhs, rhs } => {
                let op_str = match operator {
                    BinaryOp::BangEqual => "!=",
                    BinaryOp::EqualEqual => "==",
                    BinaryOp::Greater => ">",
                    BinaryOp::GreaterEqual => ">=",
                    BinaryOp::Less => "<",
                    BinaryOp::LessEqual => "<=",
                    BinaryOp::Minus => "-",
                    BinaryOp::Plus => "+",
                    BinaryOp::Slash => "/",
                    BinaryOp::Star => "*",
                };
                let expr = vec![lhs.as_ref(), rhs.as_ref()];
                return Expr::parenthesize(op_str.to_string(), expr);
            }
            Expr::Grouping(expr) => {
                let expr = vec![expr.as_ref()];
                return Expr::parenthesize("group".to_string(), expr);
            }
        }
    }

    fn parenthesize(name: String, expr: Vec<&Expr>) -> String {
        let mut builder = String::new();
        builder.push('(');
        builder.push_str(&name);
        for e in expr {
            builder.push(' ');
            builder.push_str(&e.pretty_print());
        }
        builder.push(')');

        return builder;
    }
}

#[derive(Debug, Clone)]
enum ExprLiteral {
    True,
    False,
    Nil,
    Number(f64),
    String(String),
}

// Only this is right-associative
#[derive(Debug, Clone)]
enum UnaryOp {
    Minus,
    Bang,
}

#[derive(Debug, Clone)]
enum BinaryOp {
    BangEqual,
    EqualEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,
    Minus,
    Plus,
    Slash,
    Star,
}

pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Parser { tokens, current: 0 }
    }

    fn parse(&mut self) -> Expr {
        return self.expression();
    }

    fn expression(&mut self) -> Expr {
        self.equality()
    }

    fn equality(&mut self) -> Expr {
        let mut lhs_expr = self.comparison();

        while let Some(token) = self.try_consume([TokenType::BangEqual, TokenType::EqualEqual]) {
            let token_type = token.token_type;
            let rhs_expr = self.comparison();

            lhs_expr = Expr::Binary {
                operator: match token_type {
                    TokenType::BangEqual => BinaryOp::BangEqual,
                    TokenType::EqualEqual => BinaryOp::EqualEqual,
                    _ => unreachable!(),
                },
                lhs: Box::new(lhs_expr),
                rhs: Box::new(rhs_expr),
            };
        }

        lhs_expr
    }

    fn comparison(&mut self) -> Expr {
        let mut lhs_expr = self.term();

        while let Some(token) = self.try_consume([
            TokenType::Greater,
            TokenType::GreaterEqual,
            TokenType::Less,
            TokenType::LessEqual,
        ]) {
            let token_type = token.token_type;
            let rhs_expr = self.term();

            lhs_expr = Expr::Binary {
                operator: match token_type {
                    TokenType::Greater => BinaryOp::Greater,
                    TokenType::GreaterEqual => BinaryOp::GreaterEqual,
                    TokenType::Less => BinaryOp::Less,
                    TokenType::LessEqual => BinaryOp::LessEqual,
                    _ => unreachable!(),
                },
                lhs: Box::new(lhs_expr),
                rhs: Box::new(rhs_expr),
            };
        }

        lhs_expr
    }

    fn term(&mut self) -> Expr {
        let mut lhs_expr = self.factor();

        while let Some(token) = self.try_consume([TokenType::Minus, TokenType::Plus]) {
            let token_type = token.token_type;
            let rhs_expr = self.factor();

            lhs_expr = Expr::Binary {
                operator: match token_type {
                    TokenType::Minus => BinaryOp::Minus,
                    TokenType::Plus => BinaryOp::Plus,
                    _ => unreachable!(),
                },
                lhs: Box::new(lhs_expr),
                rhs: Box::new(rhs_expr),
            };
        }

        lhs_expr
    }

    fn factor(&mut self) -> Expr {
        let mut lhs_expr = self.unary();

        while let Some(token) = self.try_consume([TokenType::Slash, TokenType::Star]) {
            let token_type = token.token_type;
            let rhs_expr = self.unary();

            lhs_expr = Expr::Binary {
                operator: match token_type {
                    TokenType::Slash => BinaryOp::Slash,
                    TokenType::Star => BinaryOp::Star,
                    _ => unreachable!(),
                },
                lhs: Box::new(lhs_expr),
                rhs: Box::new(rhs_expr),
            };
        }

        lhs_expr
    }

    fn unary(&mut self) -> Expr {
        if let Some(token) = self.try_consume([TokenType::Bang, TokenType::Minus]) {
            let expr = Box::new(self.unary());
            return Expr::Unary {
                operator: match token.token_type {
                    TokenType::Bang => UnaryOp::Bang,
                    TokenType::Minus => UnaryOp::Minus,
                    _ => unreachable!(),
                },
                expr,
            };
        }
        self.primary()
    }

    fn primary(&mut self) -> Expr {
        if let Some(token) = self.try_consume([TokenType::False]) {
            return Expr::Literal(ExprLiteral::False);
        }

        if let Some(token) = self.try_consume([TokenType::True]) {
            return Expr::Literal(ExprLiteral::True);
        }

        if let Some(token) = self.try_consume([TokenType::Nil]) {
            return Expr::Literal(ExprLiteral::Nil);
        }

        if let Some(token) = self.try_consume([TokenType::Number, TokenType::String]) {
            let literal = match &token.literal {
                Some(lexer::Literal::Number(i)) => ExprLiteral::Number(*i),
                Some(lexer::Literal::String(s)) => ExprLiteral::String(s.clone()),
                _ => unreachable!(),
            };
            return Expr::Literal(literal);
        }

        if let Some(_) = self.try_consume([TokenType::LeftParen]) {
            let expr = self.expression();
            self.try_consume([TokenType::RightParen]).unwrap();

            return Expr::Grouping(Box::new(expr));
            error(&self.peek().unwrap(), "Expect expression.".to_string());
        }
    }

    fn consume(&mut self, token: TokenType, message: String) -> Option<Token> {
        let opt = self.try_consume([token]);
        if opt.is_none() {
            error(&self.tokens[self.current], message);
        }
        opt
    }

    // Eat one token, move forward one
    fn try_consume<const N: usize>(&mut self, token_types: [TokenType; N]) -> Option<Token> {
        let current = self.peek()?;
        if token_types.contains(&current.token_type) {
            self.advance();
            return Some(current);
        }
        None
    }

    fn peek(&self) -> Option<Token> {
        let token = self.tokens.get(self.current)?;
        if token.token_type == TokenType::Eof {
            return None;
        }
        Some(token.clone())
    }

    fn is_at_end(&self) -> bool {
        self.peek()
            .is_none_or(|token| token.token_type == TokenType::Eof)
    }

    fn advance(&mut self) {
        if let Some(_) = self.peek() {
            self.current += 1;
        }
    }

    fn previous(&mut self) -> Option<Token> {
        if self.current > 0 {
            Some(self.tokens[self.current].clone())
        } else {
            None
        }
    }

    fn synchronize(&mut self) {
        self.advance();
        while !self.is_at_end() {
            if let Some(token) = self.previous() {
                if token.token_type == TokenType::Semicolon {
                    return;
                }
            };

            if let Some(token) = self.peek() {
                match token.token_type {
                    TokenType::Class => return,
                    TokenType::Fun => return,
                    TokenType::For => return,
                    TokenType::If => return,
                    TokenType::Print => return,
                    TokenType::Return => return,
                    TokenType::Var => return,
                    TokenType::While => return,
                    _ => self.advance(),
                }
            }
        }
    }
}

fn error(token: &Token, message: String) {
    if token.token_type == TokenType::Eof {
        report(token.line, " at end".to_string(), message);
    } else {
        report(token.line, format!(" at '{}'", token.lexeme), message);
    }
}
