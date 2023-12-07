use color_eyre::owo_colors::OwoColorize;
use std::{fmt::Display, str::FromStr};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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
            Operator::Addition => write!(f, "{}", "+".purple()),
            Operator::Subtraction => write!(f, "{}", "-".purple()),
            Operator::Multiplication => write!(f, "{}", "*".purple()),
            Operator::Division => write!(f, "{}", "/".purple()),
            Operator::Quotient => write!(f, "{}", "DIV".purple()),
            Operator::Modulo => write!(f, "{}", "MOD".purple()),

            Operator::And => write!(f, "{}", "AND".purple()),
            Operator::Or => write!(f, "{}", "OR".purple()),
            Operator::Not => write!(f, "{}", "NOT".purple()),

            Operator::Equals => write!(f, "{}", "=".purple()),
            Operator::NotEquals => write!(f, "{}", "<>".purple()),
            Operator::GreaterThan => write!(f, "{}", ">".purple()),
            Operator::LesserThan => write!(f, "{}", "<".purple()),
            Operator::GreaterEqual => write!(f, "{}", ">=".purple()),
            Operator::LesserEqual => write!(f, "{}", "<=".purple()),
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
