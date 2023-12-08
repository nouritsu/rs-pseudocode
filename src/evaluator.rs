use std::collections::HashMap;

use crate::{expr::Expr, value::Value};

pub fn eval(expr: &Expr, vars: HashMap<String, Value>) -> Result<Value, ()> {
    Ok(match expr {
        Expr::Literal(_) => todo!(),
        Expr::Variable(_) => todo!(),
        Expr::Negation(_) => todo!(),
        Expr::Not(_) => todo!(),
        Expr::Addition(_, _) => todo!(),
        Expr::Subtraction(_, _) => todo!(),
        Expr::Multiplication(_, _) => todo!(),
        Expr::Division(_, _) => todo!(),
        Expr::Quotient(_, _) => todo!(),
        Expr::Modulo(_, _) => todo!(),
        Expr::And(_, _) => todo!(),
        Expr::Or(_, _) => todo!(),
        Expr::Equals(_, _) => todo!(),
        Expr::NotEquals(_, _) => todo!(),
        Expr::GreaterThan(_, _) => todo!(),
        Expr::LesserThan(_, _) => todo!(),
        Expr::GreaterEqual(_, _) => todo!(),
        Expr::LesserEqual(_, _) => todo!(),
    })
}
