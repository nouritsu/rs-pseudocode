use std::fmt::Display;

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
        }
    }
}
