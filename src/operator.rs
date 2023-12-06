use std::{fmt::Display, str::FromStr};

#[derive(Debug)]
pub enum Operator {
    // Math
    Addition,
    Subtraction,
    Multiplication,
    Division,
    Quotient,
    Modulo,

    // Logical
    And,
    Or,
    Not,

    // Comparison
    Equals,
    NotEquals,
    GreaterThan,
    LesserThan,
    GreaterEqual,
    LesserEqual,
}

impl Display for Operator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            Operator::Addition => write!(f, "{}", "+"),
            Operator::Subtraction => write!(f, "{}", "-"),
            Operator::Multiplication => write!(f, "{}", "*"),
            Operator::Division => write!(f, "{}", "/"),
            Operator::Quotient => write!(f, "{}", "DIV"),
            Operator::Modulo => write!(f, "{}", "MOD"),

            Operator::And => write!(f, "{}", "AND"),
            Operator::Or => write!(f, "{}", "OR"),
            Operator::Not => write!(f, "{}", "NOT"),

            Operator::Equals => write!(f, "{}", "="),
            Operator::NotEquals => write!(f, "{}", "<>"),
            Operator::GreaterThan => write!(f, "{}", ">"),
            Operator::LesserThan => write!(f, "{}", "<"),
            Operator::GreaterEqual => write!(f, "{}", ">="),
            Operator::LesserEqual => write!(f, "{}", "<="),
        }
    }
}

impl FromStr for Operator {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "+" => Operator::Addition,
            "-" => Operator::Subtraction,
            "*" => Operator::Multiplication,
            "/" => Operator::Division,
            "DIV" => Operator::Quotient,
            "MOD" => Operator::Modulo,

            "AND" => Operator::And,
            "OR" => Operator::Or,
            "NOT" => Operator::Not,

            "=" => Operator::Equals,
            "<>" => Operator::NotEquals,
            ">" => Operator::GreaterThan,
            "<" => Operator::LesserThan,
            ">=" => Operator::GreaterEqual,
            "<=" => Operator::LesserEqual,

            _ => return Err(()),
        })
    }
}
