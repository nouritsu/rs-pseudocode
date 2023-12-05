use crate::{operator::Operator, value::Value};
use std::fmt::Display;

pub enum Expr {
    Literal(Value),
    Variable(String),
    Unary(Operator, Box<Expr>),
    Binary(Box<Expr>, Operator, Box<Expr>),
}

impl Display for Expr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Expr::Literal(l) => write!(f, "({})", l),
            Expr::Variable(s) => write!(f, "({})", s),
            Expr::Unary(o, r) => write!(f, "({} {})", o, r),
            Expr::Binary(l, o, r) => write!(f, "({} {} {})", l, o, r),
        }
    }
}
