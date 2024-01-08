#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Operator {
    // Arithmetic
    Plus,
    Minus,
    Divide,
    Multiply,
    Quotient,
    Remainder,

    // Logical
    And,
    Or,
    Not,

    // Comparison
    Gt,
    Lt,
    Ge,
    Le,
    Eq,
    Ne,
}
