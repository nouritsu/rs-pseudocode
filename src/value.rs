use std::fmt::Display;

#[derive(Debug)]
pub enum Value {
    Integer(u64),
    Real(f64),
    Character(char),
    String(String),
    Boolean(bool),
    Date(u8, u8, u16),
}

impl Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::Integer(i) => write!(f, "{}", i),
            Value::Real(n) => write!(f, "{}", n),
            Value::Character(c) => write!(f, "{}", c),
            Value::String(s) => write!(f, "{}", s),
            Value::Boolean(b) => write!(f, "{}", if *b { "TRUE" } else { "FALSE" }),
            Value::Date(d, m, y) => write!(f, "{}/{}/{}", d, m, y),
        }
    }
}
