use crate::common::{Expr, Operator, Stmt, Value};
use chumsky::prelude::*;
use time::{Date, Month};

macro_rules! pty {
    ($t: ty) => {
        impl Parser<'src, &'src str, $t, extra::Err<Rich<'src, char>>> + Clone
    };
}

pub fn parser<'src>() -> pty!(Vec<Stmt>) {
    stmt()
        .padded()
        .then_ignore(text::newline().or(end()))
        .repeated()
        .collect()
}

/* Statements */
fn stmt<'src>() -> pty!(Stmt) {
    stmt_output().or(stmt_expr())
}

fn stmt_output<'src>() -> pty!(Stmt) {
    text::keyword("OUTPUT")
        .ignore_then(expr().padded().separated_by(just(',')).collect())
        .map(Stmt::Output)
        .boxed()
}

fn stmt_expr<'src>() -> pty!(Stmt) {
    expr().map(Stmt::Expression).boxed()
}

/* Expressions */
fn expr<'src>() -> pty!(Expr) {
    let jp = |c| just(c).padded();

    let variable = text::ident()
        .and_is(kw().not())
        .map(|s: &str| s.to_owned())
        .map(Expr::Variable);

    recursive(|expr| {
        let literal = literal().map(Expr::Literal);

        let atom = literal
            .or(expr.delimited_by(just('('), just(')')))
            .or(variable)
            .boxed();

        let unary = jp("-")
            .to(Operator::Minus)
            .or(jp("NOT").to(Operator::Not))
            .repeated()
            .foldr(atom, |op, rhs| Expr::Unary(op, Box::new(rhs)))
            .boxed();

        let binary = {
            let product = unary
                .clone()
                .foldl(
                    choice((
                        jp("*").to(Operator::Multiply),
                        jp("/").to(Operator::Divide),
                        jp("DIV").to(Operator::Quotient),
                        jp("MOD").to(Operator::Remainder),
                    ))
                    .then(unary)
                    .repeated(),
                    |lhs, (op, rhs)| Expr::Binary(Box::new(lhs), op, Box::new(rhs)),
                )
                .boxed();

            let sum = product
                .clone()
                .foldl(
                    choice((jp("+").to(Operator::Plus), jp("-").to(Operator::Minus)))
                        .then(product)
                        .repeated(),
                    |lhs, (op, rhs)| Expr::Binary(Box::new(lhs), op, Box::new(rhs)),
                )
                .boxed();

            let comparison1 = sum
                .clone()
                .foldl(
                    choice((
                        jp(">=").to(Operator::Ge),
                        jp("<=").to(Operator::Le),
                        jp(">").to(Operator::Gt),
                        jp("<").to(Operator::Lt),
                    ))
                    .then(sum)
                    .repeated(),
                    |lhs, (op, rhs)| Expr::Binary(Box::new(lhs), op, Box::new(rhs)),
                )
                .boxed();

            let comparison2 = comparison1
                .clone()
                .foldl(
                    choice((jp("=").to(Operator::Eq), jp("<>").to(Operator::Ne)))
                        .then(comparison1)
                        .repeated(),
                    |lhs, (op, rhs)| Expr::Binary(Box::new(lhs), op, Box::new(rhs)),
                )
                .boxed();

            let and = comparison2
                .clone()
                .foldl(
                    jp("AND").to(Operator::And).then(comparison2).repeated(),
                    |lhs, (op, rhs)| Expr::Binary(Box::new(lhs), op, Box::new(rhs)),
                )
                .boxed();

            let or = and.clone().foldl(
                jp("OR").to(Operator::Or).then(and).repeated(),
                |lhs, (op, rhs)| Expr::Binary(Box::new(lhs), op, Box::new(rhs)),
            );

            or
        };

        binary
    })
}

/* Literals */
fn literal<'src>() -> pty!(Value) {
    let c_escape = just('\\').ignore_then(
        just('\\')
            .or(just('/'))
            .or(just('\''))
            .or(just('n').to('\n'))
            .or(just('r').to('\r'))
            .or(just('t').to('\t')),
    );

    let s_escape = just('\\').ignore_then(
        just('\\')
            .or(just('/'))
            .or(just('"'))
            .or(just('n').to('\n'))
            .or(just('r').to('\r'))
            .or(just('t').to('\t')),
    );

    let real = text::int(10)
        .then_ignore(just("."))
        .then(text::int(10))
        .map(|(w, f): (&str, &str)| format!("{}.{}", w, f).parse().unwrap())
        .map(Value::Real)
        .boxed();

    let int = text::int(10)
        .map(|s: &str| s.parse().unwrap())
        .map(Value::Integer)
        .boxed();

    let r#bool = just("TRUE")
        .or(just("FALSE"))
        .map(|s: &str| s == "TRUE")
        .map(Value::Boolean)
        .boxed();

    let r#char = none_of("\\'")
        .or(c_escape)
        .delimited_by(just('\''), just('\''))
        .map(Value::Character)
        .boxed();

    let r#str = none_of("\\\"")
        .or(s_escape)
        .repeated()
        .collect::<String>()
        .delimited_by(just('"'), just('"'))
        .map(Value::String)
        .boxed();

    let date = n_digits(2, 10)
        .then_ignore(just('/'))
        .then(n_digits(2, 10))
        .then_ignore(just('/'))
        .then(n_digits(4, 10))
        .delimited_by(just('`'), just('`'))
        .map(|((d, m), y)| {
            Date::from_calendar_date(y as i32, Month::try_from(m as u8).unwrap(), d as u8).unwrap()
        })
        .map(Value::Date)
        .boxed();

    choice((real, int, r#bool, r#char, r#str, date))
}

/* Helpers */
fn kw<'src>() -> pty!(()) {
    choice(vec![
        // IO
        text::keyword("OUTPUT"),
        text::keyword("INPUT"),
        text::keyword("OPENFILE"),
        text::keyword("CLOSEFILE"),
        text::keyword("READFILE"),
        text::keyword("WRITEFILE"),
        text::keyword("SEEK"),
        text::keyword("GETRECORD"),
        text::keyword("PUTRECORD"),
        text::keyword("READ"),
        text::keyword("WRITE"),
        text::keyword("APPEND"),
        text::keyword("RANDOM"),
        // Types
        text::keyword("INTEGER"),
        text::keyword("REAL"),
        text::keyword("BOOLEAN"),
        text::keyword("CHAR"),
        text::keyword("STRING"),
        text::keyword("DATE"),
        text::keyword("ARRAY"),
        text::keyword("TYPE"),
        text::keyword("ENDTYPE"),
        // Selection
        text::keyword("IF"),
        text::keyword("THEN"),
        text::keyword("ELSE"),
        text::keyword("ENDIF"),
        text::keyword("CASE"),
        text::keyword("OF"),
        text::keyword("OTHERWISE"),
        text::keyword("ENDCASE"),
        // Loops
        text::keyword("FOR"),
        text::keyword("TO"),
        text::keyword("STEP"),
        text::keyword("NEXT"),
        text::keyword("ENDFOR"),
        text::keyword("REPEAT"),
        text::keyword("UNTIL"),
        text::keyword("WHILE"),
        text::keyword("DO"),
        text::keyword("ENDWHILE"),
        // Declaration
        text::keyword("DECLARE"),
        text::keyword("CONSTANT"),
        // Operators
        text::keyword("MOD"),
        text::keyword("DIV"),
        text::keyword("AND"),
        text::keyword("OR"),
        text::keyword("NOT"),
        // Functions or Procedures
        text::keyword("PROCEDURE"),
        text::keyword("ENDPROCEDURE"),
        text::keyword("CALL"),
        text::keyword("FUNCTION"),
        text::keyword("RETURNS"),
        text::keyword("BYREF"),
        text::keyword("ENDFUNCTION"),
    ])
    .ignored()
}

fn n_digits<'src>(n: usize, radix: u32) -> pty!(u32) {
    text::digits(radix)
        .exactly(n)
        .collect()
        .map(|s: String| s.parse().expect("infallible"))
}
