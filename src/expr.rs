use crate::value::Value;
use color_eyre::owo_colors::OwoColorize;
use std::fmt::Debug;

#[derive(Clone, PartialEq)]
pub enum Expr {
    Literal(Value),
    Variable(String),

    /* Unary */
    Negation(Box<Expr>),
    Not(Box<Expr>),

    /* Binary */
    Addition(Box<Expr>, Box<Expr>),
    Subtraction(Box<Expr>, Box<Expr>),
    Multiplication(Box<Expr>, Box<Expr>),
    Division(Box<Expr>, Box<Expr>),
    Quotient(Box<Expr>, Box<Expr>),
    Modulo(Box<Expr>, Box<Expr>),
    And(Box<Expr>, Box<Expr>),
    Or(Box<Expr>, Box<Expr>),
    Equals(Box<Expr>, Box<Expr>),
    NotEquals(Box<Expr>, Box<Expr>),
    GreaterThan(Box<Expr>, Box<Expr>),
    LesserThan(Box<Expr>, Box<Expr>),
    GreaterEqual(Box<Expr>, Box<Expr>),
    LesserEqual(Box<Expr>, Box<Expr>),
}

impl Debug for Expr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Expr::Literal(a) => write!(f, "{}", a.red()),
            Expr::Variable(a) => write!(f, "{}", a.bright_black()),
            Expr::Negation(a) => write!(f, "({} {:?})", "-".purple(), a),
            Expr::Not(a) => write!(f, "({} {:?})", "NOT".purple(), a),
            Expr::Addition(a, b) => write!(f, "({:?} {} {:?})", a, "+".purple(), b),
            Expr::Subtraction(a, b) => write!(f, "({:?} {} {:?})", a, "-".purple(), b),
            Expr::Multiplication(a, b) => write!(f, "({:?} {} {:?})", a, "*".purple(), b),
            Expr::Division(a, b) => write!(f, "({:?} {} {:?})", a, "/".purple(), b),
            Expr::Quotient(a, b) => write!(f, "({:?} {} {:?})", a, "DIV".purple(), b),
            Expr::Modulo(a, b) => write!(f, "({:?} {} {:?})", a, "MOD".purple(), b),
            Expr::And(a, b) => write!(f, "({:?} {} {:?})", a, "AND".purple(), b),
            Expr::Or(a, b) => write!(f, "({:?} {} {:?})", a, "OR".purple(), b),
            Expr::Equals(a, b) => write!(f, "({:?} {} {:?})", a, "=".purple(), b),
            Expr::NotEquals(a, b) => write!(f, "({:?} {} {:?})", a, "<>".purple(), b),
            Expr::GreaterThan(a, b) => write!(f, "({:?} {} {:?})", a, ">".purple(), b),
            Expr::LesserThan(a, b) => write!(f, "({:?} {} {:?})", a, "<".purple(), b),
            Expr::GreaterEqual(a, b) => write!(f, "({:?} {} {:?})", a, ">=".purple(), b),
            Expr::LesserEqual(a, b) => write!(f, "({:?} {} {:?})", a, "<=".purple(), b),
        }
    }
}
