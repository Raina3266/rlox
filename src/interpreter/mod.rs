use crate::{
    error::runtime_error,
    lexer::{Token, TokenType},
    parser::{Expr, ExprLiteral},
};

#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    Nil,
    Boolean(bool),
    Number(f64),
    String(String),
}

impl Value {
    fn is_truthy(&self) -> bool {
        match self {
            Value::Nil => false,
            Value::Boolean(b) => *b,
            _ => true,
        }
    }
}

pub fn interpret(expr: &Expr) {
    match evaluate(expr) {
        Ok(value) => println!("{}", stringify(&value)),
        Err(err) => runtime_error(err),
    }
}

fn stringify(value: &Value) -> String {
    match value {
        Value::Nil => "nil".to_string(),
        Value::Boolean(bool) => bool.to_string(),
        Value::Number(num) => {
            let num = num.to_string();
            if num.ends_with(".0") {
                return num[..num.len() - 2].to_string();
            }
            num
        }
        Value::String(str) => str.clone(),
    }
}

pub fn evaluate(expr: &Expr) -> Result<Value, RuntimeError> {
    match expr {
        Expr::Literal(literal) => Ok(match literal {
            ExprLiteral::True => Value::Boolean(true),
            ExprLiteral::False => Value::Boolean(false),
            ExprLiteral::Nil => Value::Nil,
            ExprLiteral::Number(n) => Value::Number(*n),
            ExprLiteral::String(s) => Value::String(s.clone()),
        }),
        Expr::Grouping(inner) => evaluate(inner),
        Expr::Unary { operator, expr } => {
            let right = evaluate(expr)?;
            match operator.token_type {
                TokenType::Minus => {
                    let n = check_number_operand(operator, &right)?;
                    Ok(Value::Number(-n))
                }
                TokenType::Bang => Ok(Value::Boolean(!right.is_truthy())),
                _ => Err(RuntimeError {
                    token: operator.clone(),
                    message: "Unsupported unary operator.".to_string(),
                }),
            }
        }
        Expr::Binary { operator, lhs, rhs } => {
            let left = evaluate(lhs)?;
            let right = evaluate(rhs)?;
            match operator.token_type {
                TokenType::EqualEqual => Ok(Value::Boolean(left == right)),
                TokenType::BangEqual => Ok(Value::Boolean(left != right)),
                TokenType::Plus => match (&left, &right) {
                    (Value::Number(a), Value::Number(b)) => Ok(Value::Number(a + b)),
                    (Value::String(a), Value::String(b)) => Ok(Value::String(a.clone() + b)),
                    _ => Err(RuntimeError {
                        token: operator.clone(),
                        message: "Operands must be two numbers or two strings.".to_string(),
                    }),
                },
                TokenType::Minus => {
                    let (a, b) = check_number_operands(operator, &left, &right)?;
                    Ok(Value::Number(a - b))
                }
                TokenType::Slash => {
                    let (a, b) = check_number_operands(operator, &left, &right)?;
                    Ok(Value::Number(a / b))
                }
                TokenType::Star => {
                    let (a, b) = check_number_operands(operator, &left, &right)?;
                    Ok(Value::Number(a * b))
                }
                TokenType::Greater => {
                    let (a, b) = check_number_operands(operator, &left, &right)?;
                    Ok(Value::Boolean(a > b))
                }
                TokenType::GreaterEqual => {
                    let (a, b) = check_number_operands(operator, &left, &right)?;
                    Ok(Value::Boolean(a >= b))
                }
                TokenType::Less => {
                    let (a, b) = check_number_operands(operator, &left, &right)?;
                    Ok(Value::Boolean(a < b))
                }
                TokenType::LessEqual => {
                    let (a, b) = check_number_operands(operator, &left, &right)?;
                    Ok(Value::Boolean(a <= b))
                }
                _ => Err(RuntimeError {
                    token: operator.clone(),
                    message: "Unsupported binary operator.".to_string(),
                }),
            }
        }
    }
}

pub struct RuntimeError {
    pub token: Token,
    pub message: String,
}

fn check_number_operand(operator: &Token, operand: &Value) -> Result<f64, RuntimeError> {
    if let Value::Number(n) = operand {
        return Ok(*n);
    }
    Err(RuntimeError {
        token: operator.clone(),
        message: "Operand must be a number.".to_string(),
    })
}

fn check_number_operands(
    operator: &Token,
    left: &Value,
    right: &Value,
) -> Result<(f64, f64), RuntimeError> {
    if let (Value::Number(a), Value::Number(b)) = (left, right) {
        return Ok((*a, *b));
    }
    Err(RuntimeError {
        token: operator.clone(),
        message: "Operands must be numbers.".to_string(),
    })
}
