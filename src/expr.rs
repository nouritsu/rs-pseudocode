use crate::val::Value;
use color_eyre::owo_colors::OwoColorize;
use std::fmt::Debug;

#[derive(Clone, PartialEq)]
pub enum Expr {
    Literal(Value),
    Variable(String),

    /* Unary */
    Negation(Box<Self>),
    Not(Box<Self>),

    /* Binary */
    Addition(Box<Self>, Box<Self>),
    Subtraction(Box<Self>, Box<Self>),
    Multiplication(Box<Self>, Box<Self>),
    Division(Box<Self>, Box<Self>),
    Quotient(Box<Self>, Box<Self>),
    Modulo(Box<Self>, Box<Self>),
    And(Box<Self>, Box<Self>),
    Or(Box<Self>, Box<Self>),
    Equals(Box<Self>, Box<Self>),
    NotEquals(Box<Self>, Box<Self>),
    GreaterThan(Box<Self>, Box<Self>),
    LesserThan(Box<Self>, Box<Self>),
    GreaterEqual(Box<Self>, Box<Self>),
    LesserEqual(Box<Self>, Box<Self>),
}

impl Debug for Expr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Literal(a) => write!(f, "{}", a.red()),
            Self::Variable(a) => write!(f, "{}", a.bright_black()),
            Self::Negation(a) => write!(f, "({} {:?})", "-".purple(), a),
            Self::Not(a) => write!(f, "({} {:?})", "NOT".purple(), a),
            Self::Addition(a, b) => write!(f, "({:?} {} {:?})", a, "+".purple(), b),
            Self::Subtraction(a, b) => write!(f, "({:?} {} {:?})", a, "-".purple(), b),
            Self::Multiplication(a, b) => write!(f, "({:?} {} {:?})", a, "*".purple(), b),
            Self::Division(a, b) => write!(f, "({:?} {} {:?})", a, "/".purple(), b),
            Self::Quotient(a, b) => write!(f, "({:?} {} {:?})", a, "DIV".purple(), b),
            Self::Modulo(a, b) => write!(f, "({:?} {} {:?})", a, "MOD".purple(), b),
            Self::And(a, b) => write!(f, "({:?} {} {:?})", a, "AND".purple(), b),
            Self::Or(a, b) => write!(f, "({:?} {} {:?})", a, "OR".purple(), b),
            Self::Equals(a, b) => write!(f, "({:?} {} {:?})", a, "=".purple(), b),
            Self::NotEquals(a, b) => write!(f, "({:?} {} {:?})", a, "<>".purple(), b),
            Self::GreaterThan(a, b) => write!(f, "({:?} {} {:?})", a, ">".purple(), b),
            Self::LesserThan(a, b) => write!(f, "({:?} {} {:?})", a, "<".purple(), b),
            Self::GreaterEqual(a, b) => write!(f, "({:?} {} {:?})", a, ">=".purple(), b),
            Self::LesserEqual(a, b) => write!(f, "({:?} {} {:?})", a, "<=".purple(), b),
        }
    }
}
