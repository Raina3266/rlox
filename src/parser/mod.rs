#[derive(Debug, Clone)]
enum Expr {
    Literal(Literal),
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
            Expr::Literal(Literal::True) => {
                return String::from("true");
            }
            Expr::Literal(Literal::False) => {
                return String::from("false");
            }
            Expr::Literal(Literal::Nil) => {
                return String::from("nil");
            }
            Expr::Literal(Literal::Number(num)) => {
                return num.to_string();
            }
            Expr::Literal(Literal::String(str)) => {
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
enum Literal {
    True,
    False,
    Nil,
    Number(i32),
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
