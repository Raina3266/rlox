use crate::{
    lexer::{Token, TokenType}, parser::{Expr, ExprLiteral},
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
    let value = evaluate(expr);
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
                    check_number_operand(operator, &right)?;
                    let Value::Number(n) = right else {
                        unreachable!()
                    };
                    Ok(Value::Number(-n))
                }
                TokenType::Bang => Ok(Value::Boolean(!right.is_truthy())),
                _ => 
            }
        }
        Expr::Binary { operator, lhs, rhs } => {
            let left = evaluate(lhs)?;
            let right = evaluate(rhs)?;
            match operator {
                BinaryOp::EqualEqual => Ok(Value::Boolean(left == right)),
                BinaryOp::BangEqual => Ok(Value::Boolean(left != right)),
                BinaryOp::Plus => match (&left, &right) {
                    (Value::Number(a), Value::Number(b)) => Ok(Value::Number(a + b)),
                    (Value::String(a), Value::String(b)) => Ok(Value::String(a.clone() + b)),
                    _ => Err("Operands must be two numbers or two strings.".to_string()),
                },
                BinaryOp::Minus => {
                    check_number_operands(operator, &left, &right)?;
                    let (Value::Number(a), Value::Number(b)) = (left, right) else {
                        unreachable!()
                    };
                    Ok(Value::Number(a - b))
                }
                BinaryOp::Slash => {
                    check_number_operands(operator, &left, &right)?;
                    let (Value::Number(a), Value::Number(b)) = (left, right) else {
                        unreachable!()
                    };
                    Ok(Value::Number(a / b))
                }
                BinaryOp::Star => {
                    check_number_operands(operator, &left, &right)?;
                    let (Value::Number(a), Value::Number(b)) = (left, right) else {
                        unreachable!()
                    };
                    Ok(Value::Number(a * b))
                }
                BinaryOp::Greater => {
                    check_number_operands(operator, &left, &right)?;
                    let (Value::Number(a), Value::Number(b)) = (left, right) else {
                        unreachable!()
                    };
                    Ok(Value::Boolean(a > b))
                }
                BinaryOp::GreaterEqual => {
                    check_number_operands(operator, &left, &right)?;
                    let (Value::Number(a), Value::Number(b)) = (left, right) else {
                        unreachable!()
                    };
                    Ok(Value::Boolean(a >= b))
                }
                BinaryOp::Less => {
                    check_number_operands(operator, &left, &right)?;
                    let (Value::Number(a), Value::Number(b)) = (left, right) else {
                        unreachable!()
                    };
                    Ok(Value::Boolean(a < b))
                }
                BinaryOp::LessEqual => {
                    check_number_operands(operator, &left, &right)?;
                    let (Value::Number(a), Value::Number(b)) = (left, right) else {
                        unreachable!()
                    };
                    Ok(Value::Boolean(a <= b))
                }
            }
        }
    }
}

pub struct RuntimeError {
    pub token: Token,
    pub message: String,
}


fn check_number_operand(operator: &Token, operand: &Value) -> Result<(), RuntimeError> {
    if let Value::Number(_) = operand {
        return Ok(());
    }
    Err(RuntimeError {
        token: operator.clone(),
        message: "Operand must be a number.".to_string(),
    })
}

fn check_number_operands(operator: &Token, left: &Value, right: &Value) -> Result<(), RuntimeError> {
    if let (Value::Number(_), Value::Number(_)) = (left, right) {
        return Ok(());
    }
    Err(RuntimeError {
        token: operator.clone(),
        message: "Operands must be numbers.".to_string(),
    })
}
