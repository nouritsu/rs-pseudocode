use crate::{expr::Expr, val::Value, Stmt};
use chumsky::prelude::*;
use time::{Date, Month};

pub fn parser() -> impl Parser<char, Vec<Stmt>, Error = Simple<char>> {
    parse_stmt()
        .then_ignore(text::newline().or(end()))
        .repeated()
        .then_ignore(end())
}

/* Statements */
fn parse_stmt() -> impl Parser<char, Stmt, Error = Simple<char>> {
    stmt_output().or(stmt_expr())
}

fn stmt_expr() -> impl Parser<char, Stmt, Error = Simple<char>> {
    parse_expr().map(|expr| Stmt::Expression(expr))
}

fn stmt_output() -> impl Parser<char, Stmt, Error = Simple<char>> {
    text::keyword("OUTPUT")
        .ignore_then(parse_expr_args())
        .map(Stmt::Output)
}

/* Expressions */
fn parse_expr() -> impl Parser<char, Expr, Error = Simple<char>> {
    let kws = vec![
        // Operators
        "NOT",
        "OR",
        "AND",
        "DIV",
        "MOD",
        //Types
        "ARRAY",
        "INTEGER",
        "REAL",
        "CHAR",
        "STRING",
        "BOOLEAN",
        "DATE",
        // Declare Statements
        "DECLARE",
        "CONST",
        // Selection Statements
        "IF",
        "THEN",
        "ELSE",
        "ENDIF",
        "CASE",
        "OF",
        "ENDCASE",
        // Loop Statements
        "FOR",
        "TO",
        "NEXT",
        "ENDFOR",
        "WHILE",
        "DO",
        "ENDWHILE",
        "REPEAT",
        "UNTIL",
        // Functions & Procedures
        "PROCEDURE",
        "ENDPROCEDURE",
        "FUNCTION",
        "ENDFUNCTION",
        "RETURNS",
        "BYREF",
        "BYVAL",
        "OUTPUT",
        "INPUT",
        "OPENFILE",
        "CLOSEFILE",
        "READ",
        "WRITE",
        "APPEND",
        "RANDOM",
    ];

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
    let escape = just('\\').ignore_then(
        just('\\')
            .or(just('/'))
            .or(just('\''))
            .or(just('n').to('\n'))
            .or(just('r').to('\r'))
            .or(just('t').to('\t')),
    );

    none_of("\\'")
        .or(escape)
        .map(Value::Character)
        .delimited_by(just('\''), just('\''))
}

fn lit_str() -> impl Parser<char, Value, Error = Simple<char>> + Clone {
    let escape = just('\\').ignore_then(
        just('\\')
            .or(just('/'))
            .or(just('"'))
            .or(just('n').to('\n'))
            .or(just('r').to('\r'))
            .or(just('t').to('\t')),
    );

    none_of("\\\"")
        .or(escape)
        .repeated()
        .delimited_by(just('"'), just('"'))
        .map(|s| s.iter().collect())
        .map(Value::String)
}

fn lit_bool() -> impl Parser<char, Value, Error = Simple<char>> + Clone {
    just("TRUE")
        .or(just("FALSE"))
        .map(|s| Value::Boolean(s == "TRUE"))
}

fn lit_date() -> impl Parser<char, Value, Error = Simple<char>> + Clone {
    n_digits(2, 10)
        .then_ignore(just('/'))
        .then(n_digits(2, 10))
        .then_ignore(just("/"))
        .then(n_digits(4, 10))
        .map(|((d, m), y)| {
            Value::Date(
                Date::from_calendar_date(
                    y as i32,
                    Month::try_from(m as u8).unwrap(), //TODO: handle the error here, don't unwrap
                    d as u8,
                )
                .unwrap(), //TODO: handle the error here, don't unwrap
            )
        })
        .delimited_by(just('`'), just('`'))
}

/* Helpers */
fn n_digits(n: usize, radix: u32) -> impl Parser<char, u32, Error = Simple<char>> + Clone {
    filter(move |c: &char| c.is_digit(radix))
        .repeated()
        .exactly(n)
        .collect()
        .map(|s: String| s.parse().unwrap())
        .padded()
}

fn parse_expr_args() -> impl Parser<char, Vec<Expr>, Error = Simple<char>> {
    parse_expr().padded().separated_by(just(","))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Value::*;
    use time::macros::date;

    macro_rules! passes {
        ($r : expr, $x :expr) => {
            assert!($r.is_ok());
            assert_eq!($r.unwrap(), $x)
        };
    }

    macro_rules! fails {
        ($r :expr) => {
            assert!($r.is_err());
        };
    }

    mod literal {
        use super::*;

        #[test]
        fn integer() {
            let p = lit_int();

            passes!(p.parse("123"), Integer(123));
            passes!(p.parse("0"), Integer(0));

            fails!(p.parse(""));
        }

        #[test]
        fn real() {
            let p = lit_real();

            passes!(p.parse("123.123"), Real(123.123));
            passes!(p.parse("0.0"), Real(0.0));

            fails!(p.parse("123"));
            fails!(p.parse("0"));

            fails!(p.parse(""));
        }

        #[test]
        fn character() {
            let p = lit_char();

            passes!(p.parse("'a'"), Character('a'));
            passes!(p.parse("'\n'"), Character('\n'));

            fails!(p.parse("''"));
            fails!(p.parse("a"));
            fails!(p.parse("'abc'"));

            fails!(p.parse(""));
        }

        #[test]
        fn string() {
            let p = lit_str();

            passes!(p.parse("\"a\""), String("a".into()));
            passes!(p.parse("\"Hello\""), String("Hello".into()));
            passes!(p.parse("\"\n\""), String("\n".into()));
            passes!(p.parse("\"\""), String("".into()));

            fails!(p.parse("a"));

            fails!(p.parse(""));
        }

        #[test]
        fn boolean() {
            let p = lit_bool();

            passes!(p.parse("TRUE"), Boolean(true));
            passes!(p.parse("FALSE"), Boolean(false));

            fails!(p.parse("false"));
            fails!(p.parse("true"));

            fails!(p.parse("FALES"));

            fails!(p.parse(""));
        }

        #[test]
        fn date() {
            let p = lit_date();

            passes!(p.parse("`12/12/2023`"), Date(date!(2023 - 12 - 12)));

            fails!(p.parse("12/12/2023"));
            fails!(p.parse("12-12-2023"));

            fails!(p.parse("`1-12-2023`"));
            fails!(p.parse("`12-1-2023`"));
            fails!(p.parse("`12-12-23`"));

            fails!(p.parse(""));
        }
    }
}
