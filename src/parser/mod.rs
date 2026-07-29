use crate::{
    error::report,
    lexer::{self, Token, TokenType},
};

#[derive(Debug, Clone)]
pub enum Expr {
    Literal(ExprLiteral),
    Unary {
        operator: Token,
        expr: Box<Self>,
    },
    Binary {
        operator: Token,
        lhs: Box<Self>,
        rhs: Box<Self>,
    },
    Grouping(Box<Self>),
}

impl Expr {
    pub fn pretty_print(&self) -> String {
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
                let op_str = &operator.lexeme;
                let expr = vec![&**expr];
                return Expr::parenthesize(op_str.to_string(), expr);
            }
            Expr::Binary { operator, lhs, rhs } => {
                let op_str = &operator.lexeme;
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
pub enum ExprLiteral {
    True,
    False,
    Nil,
    Number(f64),
    String(String),
}

pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Parser { tokens, current: 0 }
    }

    pub fn parse(&mut self) -> Result<Expr, ()> {
        self.expression()
    }

    fn expression(&mut self) -> Result<Expr, ()> {
        self.equality()
    }

    fn equality(&mut self) -> Result<Expr, ()> {
        let mut lhs_expr = self.comparison()?;

        while let Some(token) = self.try_consume([TokenType::BangEqual, TokenType::EqualEqual]) {
            let rhs_expr = self.comparison()?;

            lhs_expr = Expr::Binary {
                operator: token,
                lhs: Box::new(lhs_expr),
                rhs: Box::new(rhs_expr),
            };
        }

        Ok(lhs_expr)
    }

    fn comparison(&mut self) -> Result<Expr, ()> {
        let mut lhs_expr = self.term()?;

        while let Some(token) = self.try_consume([
            TokenType::Greater,
            TokenType::GreaterEqual,
            TokenType::Less,
            TokenType::LessEqual,
        ]) {
            let rhs_expr = self.term()?;

            lhs_expr = Expr::Binary {
                operator: token,
                lhs: Box::new(lhs_expr),
                rhs: Box::new(rhs_expr),
            };
        }

        Ok(lhs_expr)
    }

    fn term(&mut self) -> Result<Expr, ()> {
        let mut lhs_expr = self.factor()?;

        while let Some(token) = self.try_consume([TokenType::Minus, TokenType::Plus]) {
            let rhs_expr = self.factor()?;

            lhs_expr = Expr::Binary {
                operator: token,
                lhs: Box::new(lhs_expr),
                rhs: Box::new(rhs_expr),
            };
        }

        Ok(lhs_expr)
    }

    fn factor(&mut self) -> Result<Expr, ()> {
        let mut lhs_expr = self.unary()?;

        while let Some(token) = self.try_consume([TokenType::Slash, TokenType::Star]) {
            let rhs_expr = self.factor()?;

            lhs_expr = Expr::Binary {
                operator: token,
                lhs: Box::new(lhs_expr),
                rhs: Box::new(rhs_expr),
            };
        }

        Ok(lhs_expr)
    }

    fn unary(&mut self) -> Result<Expr, ()> {
        if let Some(token) = self.try_consume([TokenType::Bang, TokenType::Minus]) {
            let expr = Box::new(self.unary()?);
            return Ok(Expr::Unary {
                operator: token,
                expr,
            });
        }
        self.primary()
    }

    fn primary(&mut self) -> Result<Expr, ()> {
        if let Some(_) = self.try_consume([TokenType::False]) {
            return Ok(Expr::Literal(ExprLiteral::False));
        } else if let Some(_) = self.try_consume([TokenType::True]) {
            return Ok(Expr::Literal(ExprLiteral::True));
        } else if let Some(_) = self.try_consume([TokenType::Nil]) {
            return Ok(Expr::Literal(ExprLiteral::Nil));
        } else if let Some(token) = self.try_consume([TokenType::Number, TokenType::String]) {
            let literal = match &token.literal {
                Some(lexer::Literal::Number(i)) => ExprLiteral::Number(*i),
                Some(lexer::Literal::String(s)) => ExprLiteral::String(s.clone()),
                _ => unreachable!(),
            };
            return Ok(Expr::Literal(literal));
        } else {
            if let Some(_) = self.try_consume([TokenType::LeftParen]) {
                let expr = self.expression()?;
                self.try_consume([TokenType::RightParen]).unwrap();

                return Ok(Expr::Grouping(Box::new(expr)));
            }
        }

        error(&self.peek().unwrap(), "Expect expression.".to_string());
        Err(())
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
