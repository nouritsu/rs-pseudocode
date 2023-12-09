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
        Expr::Equals(a, b) => eval(a, vars)?.eq(&eval(b, vars)?),
        Expr::NotEquals(a, b) => eval(a, vars)?.ne(&eval(b, vars)?),
        Expr::GreaterThan(a, b) => eval(a, vars)?.gt(&eval(b, vars)?),
        Expr::LesserThan(a, b) => eval(a, vars)?.lt(&eval(b, vars)?),
        Expr::GreaterEqual(a, b) => eval(a, vars)?.ge(&eval(b, vars)?),
        Expr::LesserEqual(a, b) => eval(a, vars)?.le(&eval(b, vars)?),
    }
    .map_err(|_| ())
}
