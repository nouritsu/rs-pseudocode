use std::collections::HashMap;

use crate::common::{Operator, Value, ValueError};

#[derive(Debug, Clone, PartialEq)]
pub enum Expr {
    Literal(Value),
    Variable(String),
    Unary(Operator, Box<Self>),
    Binary(Box<Self>, Operator, Box<Self>),
}

#[derive(Debug)]
pub enum Error {
    VariableNotFound(String),
    ValueError(ValueError),
}

impl Expr {
    pub fn eval(&self, env: &HashMap<String, Value>) -> Result<Value, Error> {
        match self {
            Self::Literal(v) => Ok(v.clone()),

            Self::Variable(name) => match env.get(name) {
                Some(v) => Ok(v.clone()),
                None => return Err(Error::VariableNotFound(name.clone())),
            },

            Self::Unary(op, a) => {
                let a = a.eval(env)?;

                match op {
                    Operator::Minus => a.neg(),
                    Operator::Not => a.not(),
                    _ => unreachable!(),
                }
            }

            Self::Binary(a, op, b) => {
                let a = a.eval(env)?;
                let b = b.eval(env)?;

                match op {
                    Operator::Plus => a.add(&b),
                    Operator::Minus => a.sub(&b),
                    Operator::Divide => a.div(&b),
                    Operator::Multiply => a.mul(&b),
                    Operator::Quotient => a.quot(&b),
                    Operator::Remainder => a.modu(&b),
                    Operator::And => a.and(&b),
                    Operator::Or => a.or(&b),
                    Operator::Gt => a.gt(&b),
                    Operator::Lt => a.lt(&b),
                    Operator::Ge => a.ge(&b),
                    Operator::Le => a.le(&b),
                    Operator::Eq => a.eq(&b),
                    Operator::Ne => a.ne(&b),
                    _ => unreachable!(),
                }
            }
        }
        .map_err(Error::ValueError)
    }
}
