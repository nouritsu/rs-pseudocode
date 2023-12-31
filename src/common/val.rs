use std::fmt::Display;

use time::Date;

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum Value {
    Integer(i64),
    Real(f64),
    Character(char),
    String(String),
    Boolean(bool),
    Date(Date),
}

impl Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::Integer(i) => write!(f, "{}", i),
            Value::Real(r) => write!(f, "{}", r),
            Value::Character(c) => write!(f, "{}", c),
            Value::String(s) => write!(f, "{}", s),
            Value::Boolean(b) => write!(f, "{}", b),
            Value::Date(d) => write!(f, "{} {} {}", d.day(), d.month(), d.year()),
        }
    }
}

#[derive(Debug)]
pub enum Error {
    InvalidType,
    InvalidOperation,
}

pub type ValueResult<T> = Result<T, Error>;

// Try As Methods
impl Value {
    fn try_as_integer(&self) -> ValueResult<i64> {
        match self {
            Self::Integer(i) => Ok(*i),
            _ => Err(Error::InvalidType),
        }
    }

    fn try_as_real(&self) -> ValueResult<f64> {
        match self {
            Self::Real(r) => Ok(*r),
            _ => Err(Error::InvalidType),
        }
    }

    fn try_as_character(&self) -> ValueResult<char> {
        match self {
            Self::Character(c) => Ok(*c),
            _ => Err(Error::InvalidType),
        }
    }

    fn try_as_string(&self) -> ValueResult<String> {
        match self {
            Self::String(s) => Ok(s.clone()),
            _ => Err(Error::InvalidType),
        }
    }

    fn try_as_boolean(&self) -> ValueResult<bool> {
        match self {
            Self::Boolean(b) => Ok(*b),
            _ => Err(Error::InvalidType),
        }
    }

    fn try_as_date(&self) -> ValueResult<Date> {
        match self {
            Self::Date(d) => Ok(*d),
            _ => Err(Error::InvalidType),
        }
    }
}

// Operations
impl Value {
    pub fn neg(&self) -> ValueResult<Value> {
        Ok(match self {
            Self::Integer(i) => Value::Integer(-i),
            Self::Real(r) => Value::Real(-r),
            _ => return Err(Error::InvalidOperation),
        })
    }

    pub fn not(&self) -> ValueResult<Value> {
        Ok(match self {
            Self::Boolean(b) => Value::Boolean(!b),
            _ => return Err(Error::InvalidOperation),
        })
    }

    pub fn add(&self, rhs: &Self) -> ValueResult<Value> {
        Ok(match self {
            Self::Integer(a) => match rhs {
                Self::Integer(b) => Value::Integer(a + b),
                Self::Real(b) => Value::Real(*a as f64 + b),
                _ => return Err(Error::InvalidOperation),
            },

            Self::Real(a) => match rhs {
                Self::Integer(b) => Value::Real(a + *b as f64),
                Self::Real(b) => Value::Real(a + b),
                _ => return Err(Error::InvalidOperation),
            },

            Self::String(a) => match rhs {
                Self::String(b) => Value::String(a.to_owned() + b),
                Self::Character(b) => Value::String(a.to_owned() + &b.to_string()),
                _ => return Err(Error::InvalidOperation),
            },

            Self::Character(a) => match rhs {
                Self::Character(b) => Value::String(a.to_string() + &b.to_string()),
                Self::String(b) => Value::String(a.to_string() + b),
                _ => return Err(Error::InvalidOperation),
            },

            _ => return Err(Error::InvalidOperation),
        })
    }

    pub fn sub(&self, rhs: &Self) -> ValueResult<Value> {
        Ok(match self {
            Self::Integer(a) => match rhs {
                Self::Integer(b) => Value::Integer(a - b),
                Self::Real(b) => Value::Real(*a as f64 - b),
                _ => return Err(Error::InvalidOperation),
            },

            Self::Real(a) => match rhs {
                Self::Integer(b) => Value::Real(a - *b as f64),
                Self::Real(b) => Value::Real(a - b),
                _ => return Err(Error::InvalidOperation),
            },

            _ => return Err(Error::InvalidOperation),
        })
    }

    pub fn mul(&self, rhs: &Self) -> ValueResult<Value> {
        Ok(match self {
            Self::Integer(a) => match rhs {
                Self::Integer(b) => Value::Integer(a * b),
                Self::Real(b) => Value::Real(*a as f64 * b),
                _ => return Err(Error::InvalidOperation),
            },

            Self::Real(a) => match rhs {
                Self::Integer(b) => Value::Real(a * *b as f64),
                Self::Real(b) => Value::Real(a * b),
                _ => return Err(Error::InvalidOperation),
            },

            _ => return Err(Error::InvalidOperation),
        })
    }

    pub fn div(&self, rhs: &Self) -> ValueResult<Value> {
        Ok(match self {
            Self::Integer(a) => match rhs {
                Self::Integer(b) => Value::Real(*a as f64 / *b as f64),
                Self::Real(b) => Value::Real(*a as f64 / b),
                _ => return Err(Error::InvalidOperation),
            },

            Self::Real(a) => match rhs {
                Self::Integer(b) => Value::Real(a / *b as f64),
                Self::Real(b) => Value::Real(a / b),
                _ => return Err(Error::InvalidOperation),
            },

            _ => return Err(Error::InvalidOperation),
        })
    }

    pub fn quot(&self, rhs: &Self) -> ValueResult<Value> {
        Ok(match self {
            Self::Integer(a) => match rhs {
                Self::Integer(b) => Value::Integer(a / b),
                Self::Real(b) => Value::Integer((*a as f64 / b) as i64),
                _ => return Err(Error::InvalidOperation),
            },

            Self::Real(a) => match rhs {
                Self::Integer(b) => Value::Integer((a / *b as f64) as i64),
                Self::Real(b) => Value::Integer((a / b) as i64),
                _ => return Err(Error::InvalidOperation),
            },

            _ => return Err(Error::InvalidOperation),
        })
    }

    pub fn modu(&self, rhs: &Self) -> ValueResult<Value> {
        Ok(match self {
            Self::Integer(a) => match rhs {
                Self::Integer(b) => Value::Integer(a % b),
                Self::Real(b) => Value::Real(*a as f64 % b),
                _ => return Err(Error::InvalidOperation),
            },

            Self::Real(a) => match rhs {
                Self::Integer(b) => Value::Real(a % *b as f64),
                Self::Real(b) => Value::Real(a % b),
                _ => return Err(Error::InvalidOperation),
            },

            _ => return Err(Error::InvalidOperation),
        })
    }

    pub fn and(&self, rhs: &Self) -> ValueResult<Value> {
        Ok(match self {
            Self::Boolean(a) => match rhs {
                Self::Boolean(b) => Value::Boolean(a & b),
                _ => return Err(Error::InvalidOperation),
            },

            _ => return Err(Error::InvalidOperation),
        })
    }

    pub fn or(&self, rhs: &Self) -> ValueResult<Value> {
        Ok(match self {
            Self::Boolean(a) => match rhs {
                Self::Boolean(b) => Value::Boolean(a | b),
                _ => return Err(Error::InvalidOperation),
            },

            _ => return Err(Error::InvalidOperation),
        })
    }

    pub fn eq(&self, rhs: &Self) -> ValueResult<Value> {
        Ok(match self {
            Self::Integer(a) => match rhs {
                Self::Integer(b) => Self::Boolean(a == b),
                Self::Real(b) => Self::Boolean(*a as f64 == *b),
                _ => return Err(Error::InvalidOperation),
            },
            Self::Real(a) => match rhs {
                Self::Integer(b) => Self::Boolean(*a == *b as f64),
                Self::Real(b) => Self::Boolean(a == b),
                _ => return Err(Error::InvalidOperation),
            },
            Self::Character(a) => match rhs {
                Self::Character(b) => Self::Boolean(a == b),
                _ => return Err(Error::InvalidOperation),
            },
            Self::String(a) => match rhs {
                Self::String(b) => Self::Boolean(a == b),
                _ => return Err(Error::InvalidOperation),
            },
            Self::Boolean(a) => match rhs {
                Self::Boolean(b) => Self::Boolean(a == b),
                _ => return Err(Error::InvalidOperation),
            },
            Self::Date(a) => match rhs {
                Self::Date(b) => Self::Boolean(a == b),
                _ => return Err(Error::InvalidOperation),
            },
        })
    }

    pub fn ne(&self, rhs: &Self) -> ValueResult<Value> {
        Ok(match self {
            Self::Integer(a) => match rhs {
                Self::Integer(b) => Self::Boolean(a != b),
                Self::Real(b) => Self::Boolean(*a as f64 != *b),
                _ => return Err(Error::InvalidOperation),
            },
            Self::Real(a) => match rhs {
                Self::Integer(b) => Self::Boolean(*a != *b as f64),
                Self::Real(b) => Self::Boolean(a != b),
                _ => return Err(Error::InvalidOperation),
            },
            Self::Character(a) => match rhs {
                Self::Character(b) => Self::Boolean(a != b),
                _ => return Err(Error::InvalidOperation),
            },
            Self::String(a) => match rhs {
                Self::String(b) => Self::Boolean(a != b),
                _ => return Err(Error::InvalidOperation),
            },
            Self::Boolean(a) => match rhs {
                Self::Boolean(b) => Self::Boolean(a != b),
                _ => return Err(Error::InvalidOperation),
            },
            Self::Date(a) => match rhs {
                Self::Date(b) => Self::Boolean(a != b),
                _ => return Err(Error::InvalidOperation),
            },
        })
    }

    pub fn gt(&self, rhs: &Self) -> ValueResult<Value> {
        Ok(match self {
            Self::Integer(a) => match rhs {
                Self::Integer(b) => Self::Boolean(a > b),
                Self::Real(b) => Self::Boolean(*a as f64 > *b),
                _ => return Err(Error::InvalidOperation),
            },
            Self::Real(a) => match rhs {
                Self::Integer(b) => Self::Boolean(*a > *b as f64),
                Self::Real(b) => Self::Boolean(a > b),
                _ => return Err(Error::InvalidOperation),
            },
            Self::Character(a) => match rhs {
                Self::Character(b) => Self::Boolean(a > b),
                _ => return Err(Error::InvalidOperation),
            },
            Self::String(a) => match rhs {
                Self::String(b) => Self::Boolean(a > b),
                _ => return Err(Error::InvalidOperation),
            },
            Self::Boolean(a) => match rhs {
                Self::Boolean(b) => Self::Boolean(a > b),
                _ => return Err(Error::InvalidOperation),
            },
            Self::Date(a) => match rhs {
                Self::Date(b) => Self::Boolean(a > b),
                _ => return Err(Error::InvalidOperation),
            },
        })
    }

    pub fn lt(&self, rhs: &Self) -> ValueResult<Value> {
        Ok(match self {
            Self::Integer(a) => match rhs {
                Self::Integer(b) => Self::Boolean(a < b),
                Self::Real(b) => Self::Boolean((*a as f64) < *b),
                _ => return Err(Error::InvalidOperation),
            },
            Self::Real(a) => match rhs {
                Self::Integer(b) => Self::Boolean(*a < *b as f64),
                Self::Real(b) => Self::Boolean(a < b),
                _ => return Err(Error::InvalidOperation),
            },
            Self::Character(a) => match rhs {
                Self::Character(b) => Self::Boolean(a < b),
                _ => return Err(Error::InvalidOperation),
            },
            Self::String(a) => match rhs {
                Self::String(b) => Self::Boolean(a < b),
                _ => return Err(Error::InvalidOperation),
            },
            Self::Boolean(a) => match rhs {
                Self::Boolean(b) => Self::Boolean(a < b),
                _ => return Err(Error::InvalidOperation),
            },
            Self::Date(a) => match rhs {
                Self::Date(b) => Self::Boolean(a < b),
                _ => return Err(Error::InvalidOperation),
            },
        })
    }

    pub fn ge(&self, rhs: &Self) -> ValueResult<Value> {
        Ok(match self {
            Self::Integer(a) => match rhs {
                Self::Integer(b) => Self::Boolean(a >= b),
                Self::Real(b) => Self::Boolean(*a as f64 >= *b),
                _ => return Err(Error::InvalidOperation),
            },
            Self::Real(a) => match rhs {
                Self::Integer(b) => Self::Boolean(*a >= *b as f64),
                Self::Real(b) => Self::Boolean(a >= b),
                _ => return Err(Error::InvalidOperation),
            },
            Self::Character(a) => match rhs {
                Self::Character(b) => Self::Boolean(a >= b),
                _ => return Err(Error::InvalidOperation),
            },
            Self::String(a) => match rhs {
                Self::String(b) => Self::Boolean(a >= b),
                _ => return Err(Error::InvalidOperation),
            },
            Self::Boolean(a) => match rhs {
                Self::Boolean(b) => Self::Boolean(a >= b),
                _ => return Err(Error::InvalidOperation),
            },
            Self::Date(a) => match rhs {
                Self::Date(b) => Self::Boolean(a >= b),
                _ => return Err(Error::InvalidOperation),
            },
        })
    }

    pub fn le(&self, rhs: &Self) -> ValueResult<Value> {
        Ok(match self {
            Self::Integer(a) => match rhs {
                Self::Integer(b) => Self::Boolean(a <= b),
                Self::Real(b) => Self::Boolean(*a as f64 <= *b),
                _ => return Err(Error::InvalidOperation),
            },
            Self::Real(a) => match rhs {
                Self::Integer(b) => Self::Boolean(*a <= *b as f64),
                Self::Real(b) => Self::Boolean(a <= b),
                _ => return Err(Error::InvalidOperation),
            },
            Self::Character(a) => match rhs {
                Self::Character(b) => Self::Boolean(a <= b),
                _ => return Err(Error::InvalidOperation),
            },
            Self::String(a) => match rhs {
                Self::String(b) => Self::Boolean(a <= b),
                _ => return Err(Error::InvalidOperation),
            },
            Self::Boolean(a) => match rhs {
                Self::Boolean(b) => Self::Boolean(a <= b),
                _ => return Err(Error::InvalidOperation),
            },
            Self::Date(a) => match rhs {
                Self::Date(b) => Self::Boolean(a <= b),
                _ => return Err(Error::InvalidOperation),
            },
        })
    }
}
