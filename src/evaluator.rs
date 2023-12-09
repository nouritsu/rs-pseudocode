use std::collections::HashMap;

use crate::{expr::Expr, value::Value};

pub fn eval(expr: &Expr, vars: &mut HashMap<String, Value>) -> Result<Value, ()> {
    match expr {
        Expr::Literal(l) => Ok(l.clone()),
        Expr::Variable(v) => Ok(vars.get(v).ok_or(())?.clone()),
        Expr::Negation(a) => eval(a, vars)?.neg(),
        Expr::Not(a) => eval(a, vars)?.not(),
        Expr::Addition(a, b) => eval(a, vars)?.add(&eval(b, vars)?),
        Expr::Subtraction(a, b) => eval(a, vars)?.sub(&eval(b, vars)?),
        Expr::Multiplication(a, b) => eval(a, vars)?.mul(&eval(b, vars)?),
        Expr::Division(a, b) => eval(a, vars)?.div(&eval(b, vars)?),
        Expr::Quotient(a, b) => eval(a, vars)?.quot(&eval(b, vars)?),
        Expr::Modulo(a, b) => eval(a, vars)?.modu(&eval(b, vars)?),
        Expr::And(a, b) => eval(a, vars)?.and(&eval(b, vars)?),
        Expr::Or(a, b) => eval(a, vars)?.or(&eval(b, vars)?),
        Expr::Equals(_, _) => todo!(),
        Expr::NotEquals(_, _) => todo!(),
        Expr::GreaterThan(_, _) => todo!(),
        Expr::LesserThan(_, _) => todo!(),
        Expr::GreaterEqual(_, _) => todo!(),
        Expr::LesserEqual(_, _) => todo!(),
    }
    .map_err(|_| ())
}
