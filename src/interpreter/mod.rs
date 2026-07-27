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

pub fn evaluate(expr: &Expr) -> Result<Value, String> {
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
            match operator {
                UnaryOp::Minus => {
                    check_number_operand(operator, &right)?;
                    let Value::Number(n) = right else {
                        unreachable!()
                    };
                    Ok(Value::Number(-n))
                }
                UnaryOp::Bang => Ok(Value::Boolean(!right.is_truthy())),
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

fn check_number_operand(_operator: &UnaryOp, operand: &Value) -> Result<(), String> {
    if let Value::Number(_) = operand {
        return Ok(());
    }
    Err("Operand must be a number.".to_string())
}

fn check_number_operands(_operator: &BinaryOp, left: &Value, right: &Value) -> Result<(), String> {
    if let (Value::Number(_), Value::Number(_)) = (left, right) {
        return Ok(());
    }
    Err("Operands must be numbers.".to_string())
}
