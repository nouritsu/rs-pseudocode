use crate::{expr::Expr, value::Value};
use chumsky::prelude::*;

pub fn parser() -> impl Parser<char, Expr, Error = Simple<char>> {
    parse_expr().padded().then_ignore(end())
}

/* Expressions */
fn parse_expr() -> impl Parser<char, Expr, Error = Simple<char>> {
    recursive(|expr| {
        let literal = lit().map(Expr::Literal);
        let variable = text::ident().map(Expr::Variable);
        let atom = literal
            .or(expr.delimited_by(just('('), just(')')))
            .or(variable);

        let op = |s| just(s).padded();

        let unary = op("-")
            .or(op("NOT"))
            .repeated()
            .then(atom.clone())
            .foldr(|op, rhs| match op {
                "-" => Expr::Negation(Box::new(rhs)),
                "NOT" => Expr::Not(Box::new(rhs)),
                _ => unreachable!(),
            });

        let binary = {
            let product = unary
                .clone()
                .then(
                    op("*")
                        .to(Expr::Multiplication as fn(_, _) -> _)
                        .or(op("/").to(Expr::Division as fn(_, _) -> _))
                        .or(op("DIV").to(Expr::Quotient as fn(_, _) -> _))
                        .or(op("MOD").to(Expr::Modulo as fn(_, _) -> _))
                        .then(unary)
                        .repeated(),
                )
                .foldl(|lhs, (op, rhs)| op(Box::new(lhs), Box::new(rhs)));

            let sum = product
                .clone()
                .then(
                    op("+")
                        .to(Expr::Addition as fn(_, _) -> _)
                        .or(op("-").to(Expr::Subtraction as fn(_, _) -> _))
                        .then(product)
                        .repeated(),
                )
                .foldl(|lhs, (op, rhs)| op(Box::new(lhs), Box::new(rhs)));

            let comparison1 = sum
                .clone()
                .then(
                    op(">=")
                        .to(Expr::GreaterEqual as fn(_, _) -> _)
                        .or(op("<=").to(Expr::LesserEqual as fn(_, _) -> _))
                        .or(op(">").to(Expr::GreaterThan as fn(_, _) -> _))
                        .or(op("<").to(Expr::LesserThan as fn(_, _) -> _))
                        .then(sum)
                        .repeated(),
                )
                .foldl(|lhs, (op, rhs)| op(Box::new(lhs), Box::new(rhs)));

            let comparison2 = comparison1
                .clone()
                .then(
                    op("=")
                        .to(Expr::Equals as fn(_, _) -> _)
                        .or(op("<>").to(Expr::NotEquals as fn(_, _) -> _))
                        .then(comparison1)
                        .repeated(),
                )
                .foldl(|lhs, (op, rhs)| op(Box::new(lhs), Box::new(rhs)));

            let and = comparison2
                .clone()
                .then(
                    op("AND")
                        .to(Expr::And as fn(_, _) -> _)
                        .then(comparison2)
                        .repeated(),
                )
                .foldl(|lhs, (op, rhs)| op(Box::new(lhs), Box::new(rhs)));

            let or = and
                .clone()
                .then(op("OR").to(Expr::Or as fn(_, _) -> _).then(and).repeated())
                .foldl(|lhs, (op, rhs)| op(Box::new(lhs), Box::new(rhs)));

            or
        };

        binary
    })
}

/* Literals */
fn lit() -> impl Parser<char, Value, Error = Simple<char>> + Clone {
    lit_real()
        .or(lit_int())
        .or(lit_char())
        .or(lit_str())
        .or(lit_bool())
        .or(lit_date())
}

fn lit_real() -> impl Parser<char, Value, Error = Simple<char>> + Clone {
    text::int(10)
        .then_ignore(just('.'))
        .then(text::int(10))
        .map(|(w, h)| Value::Real(format!("{}.{}", w, h).parse().unwrap()))
}

fn lit_int() -> impl Parser<char, Value, Error = Simple<char>> + Clone {
    text::int(10).map(|s: String| Value::Integer(s.parse().unwrap()))
}

fn lit_char() -> impl Parser<char, Value, Error = Simple<char>> + Clone {
    any()
        .map(Value::Character)
        .delimited_by(just('\''), just('\''))
}

fn lit_str() -> impl Parser<char, Value, Error = Simple<char>> + Clone {
    none_of("\\\"")
        .repeated()
        .map(|cs| Value::String(cs.iter().collect()))
        .delimited_by(just('"'), just('"'))
}

fn lit_bool() -> impl Parser<char, Value, Error = Simple<char>> + Clone {
    just("TRUE")
        .or(just("FALSE"))
        .map(|s| Value::Boolean(s == "TRUE"))
}

fn lit_date() -> impl Parser<char, Value, Error = Simple<char>> + Clone {
    text::digits(10)
        .then_ignore(just('/'))
        .then(text::digits(10))
        .then_ignore(just("/"))
        .then(text::digits(10))
        .map(|((d, m), y)| Value::Date(d.parse().unwrap(), m.parse().unwrap(), y.parse().unwrap()))
        .delimited_by(just('`'), just('`'))
}
