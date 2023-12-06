use crate::{expr::Expr, value::Value};
use chumsky::{prelude::*, text::digits};

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

        let unary = just("-")
            .padded()
            .or(just("NOT").padded())
            .repeated()
            .then(atom)
            .foldr(|op, rhs| Expr::Unary(op.parse().unwrap(), Box::new(rhs)));

        unary
    })
}

/* Literals */
fn lit() -> impl Parser<char, Value, Error = Simple<char>> {
    lit_real()
        .or(lit_int())
        .or(lit_char())
        .or(lit_str())
        .or(lit_bool())
        .or(lit_date())
}

fn lit_real() -> impl Parser<char, Value, Error = Simple<char>> {
    text::int(10)
        .then_ignore(just('.'))
        .then(text::int(10))
        .map(|(w, h)| Value::Real(format!("{}.{}", w, h).parse().unwrap()))
}

fn lit_int() -> impl Parser<char, Value, Error = Simple<char>> {
    text::int(10).map(|s: String| Value::Integer(s.parse().unwrap()))
}

fn lit_char() -> impl Parser<char, Value, Error = Simple<char>> {
    any()
        .map(Value::Character)
        .delimited_by(just('\''), just('\''))
}

fn lit_str() -> impl Parser<char, Value, Error = Simple<char>> {
    none_of("\\\"")
        .repeated()
        .map(|cs| Value::String(cs.iter().collect()))
        .delimited_by(just('"'), just('"'))
}

fn lit_bool() -> impl Parser<char, Value, Error = Simple<char>> {
    just("TRUE")
        .or(just("FALSE"))
        .map(|s| Value::Boolean(s == "TRUE"))
}

fn lit_date() -> impl Parser<char, Value, Error = Simple<char>> {
    text::int(10)
        .then_ignore(just('/'))
        .then(text::int(10))
        .then_ignore(just("/"))
        .then(digits(10))
        .map(|((d, m), y)| Value::Date(d.parse().unwrap(), m.parse().unwrap(), y.parse().unwrap()))
        .delimited_by(just('`'), just('`'))
}
