use crate::parser::{BinaryOp, Expr, ExprLiteral, UnaryOp};

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

pub struct Interpreter;

impl Interpreter {
    pub fn evaluate(expr: &Expr) -> Result<Value, String> {
        match expr {
            Expr::Literal(literal) => Ok(match literal {
                ExprLiteral::True => Value::Boolean(true),
                ExprLiteral::False => Value::Boolean(false),
                ExprLiteral::Nil => Value::Nil,
                ExprLiteral::Number(n) => Value::Number(*n),
                ExprLiteral::String(s) => Value::String(s.clone()),
            }),
            Expr::Grouping(inner) => Self::evaluate(inner),
            Expr::Unary { operator, expr } => {
                let right = Self::evaluate(expr)?;
                match operator {
                    UnaryOp::Minus => match right {
                        Value::Number(n) => Ok(Value::Number(-n)),
                        _ => Err("Operand must be a number.".to_string()),
                    },
                    UnaryOp::Bang => Ok(Value::Boolean(!right.is_truthy())),
                }
            }
            Expr::Binary { operator, lhs, rhs } => {
                let left = Self::evaluate(lhs)?;
                let right = Self::evaluate(rhs)?;
                match (operator, left, right) {
                    (BinaryOp::EqualEqual, left, right) => Ok(Value::Boolean(left == right)),
                    (BinaryOp::BangEqual, left, right) => Ok(Value::Boolean(left != right)),
                    (BinaryOp::Plus, Value::Number(a), Value::Number(b)) => {
                        Ok(Value::Number(a + b))
                    }
                    (BinaryOp::Plus, Value::String(a), Value::String(b)) => {
                        Ok(Value::String(a + &b))
                    }
                    (BinaryOp::Plus, _, _) => {
                        Err("Operands must be two numbers or two strings.".to_string())
                    }
                    (BinaryOp::Minus, Value::Number(a), Value::Number(b)) => {
                        Ok(Value::Number(a - b))
                    }
                    (BinaryOp::Star, Value::Number(a), Value::Number(b)) => {
                        Ok(Value::Number(a * b))
                    }
                    (BinaryOp::Slash, Value::Number(a), Value::Number(b)) => {
                        Ok(Value::Number(a / b))
                    }
                    (BinaryOp::Greater, Value::Number(a), Value::Number(b)) => {
                        Ok(Value::Boolean(a > b))
                    }
                    (BinaryOp::GreaterEqual, Value::Number(a), Value::Number(b)) => {
                        Ok(Value::Boolean(a >= b))
                    }
                    (BinaryOp::Less, Value::Number(a), Value::Number(b)) => {
                        Ok(Value::Boolean(a < b))
                    }
                    (BinaryOp::LessEqual, Value::Number(a), Value::Number(b)) => {
                        Ok(Value::Boolean(a <= b))
                    }
                    _ => Err("Operands must be numbers.".to_string()),
                }
            }
        }
    }
}
