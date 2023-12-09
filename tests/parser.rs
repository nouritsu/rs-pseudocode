#[allow(unused_macros)]
macro_rules! parse_passing {
    ($src: literal, $b: expr ) => {{
        use chumsky::Parser;
        use rs_pseudocode::parser;

        let res = parser().parse($src);
        assert!(res.is_ok());
        assert_eq!(res.unwrap(), $b);
    }};
}

#[allow(unused_macros)]
macro_rules! parse_failing {
    ($src: literal) => {{
        use chumsky::Parser;
        use rs_pseudocode::parser;

        let res = parser().parse($src);
        assert!(res.is_err());
    }};
}

mod expr {
    use rs_pseudocode::{Expr, Value};
    use time::macros::date;

    #[test]
    fn literal_expr() {
        parse_passing!("5", Expr::Literal(Value::Integer(5)));
        parse_passing!("5.0", Expr::Literal(Value::Real(5.0)));
        parse_passing!("'a'", Expr::Literal(Value::Character('a')));
        parse_passing!("\"String\"", Expr::Literal(Value::String("String".into())));
        parse_passing!("TRUE", Expr::Literal(Value::Boolean(true)));
        parse_passing!(
            "`08/12/2023`",
            Expr::Literal(Value::Date(date!(2023 - 12 - 08)))
        );

        parse_failing!("123abc999");
        parse_failing!("5.");
        parse_failing!(".5");
        parse_failing!("'Hello'");
        parse_failing!("\"String");
        parse_failing!("`1a0/10a0/2000a0");
        parse_failing!("1a0/10a0/2000a0");
    }
}
