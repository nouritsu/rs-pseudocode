/*
! NOTE: THIS FILE IS CURRENTLY NON FUNCTIONAL
! All tests in this file need to be moved to src/parser.rs, since they are unit tests
*/

// #![allow(unused_imports)]
// #![allow(unused_macros)]
// use rs_pseudocode::{Expr::*, Value::*};
// use time::macros::date;

// // Parse returns Ok and matches b
// macro_rules! parse_passing {
//     ($src: literal, $b: expr ) => {{
//         use chumsky::Parser;
//         use rs_pseudocode::parser;

//         let res = parser().parse($src);
//         assert!(res.is_ok());
//         assert_eq!(res.unwrap(), $b);
//     }};
// }

// // Parse returns ok but does not match b
// macro_rules! parse_mismatch {
//     ($src: literal, $b: expr ) => {{
//         use chumsky::Parser;
//         use rs_pseudocode::parser;

//         let res = parser().parse($src);
//         assert!(res.is_ok());
//         assert_ne!(res.unwrap(), $b);
//     }};
// }

// // Parse returns err
// macro_rules! parse_failing {
//     ($src: literal) => {{
//         use chumsky::Parser;
//         use rs_pseudocode::parser;

//         let res = parser().parse($src);
//         assert!(res.is_err());
//     }};
// }

// mod expr {
//     use super::*;

//     mod literal {
//         use super::*;

//         #[test]
//         fn integer() {
//             parse_passing!("123", Literal(Integer(123)));
//             parse_passing!("0", Literal(Integer(0)));

//             parse_mismatch!("123.123", Literal(Integer(0)));
//             parse_mismatch!("abc", Literal(Integer(0)));

//             parse_failing!("");
//         }

//         #[test]
//         fn real() {
//             parse_passing!("123.123", Literal(Real(123.123)));
//             parse_passing!("0.0", Literal(Real(0.0)));

//             parse_mismatch!("123", Literal(Real(123.0)));
//             parse_mismatch!("abc", Literal(Real(0.0)));

//             parse_failing!("0.");
//             parse_failing!(".0");
//             parse_failing!("");
//         }

//         #[test]
//         fn character() {
//             parse_passing!("'a'", Literal(Character('a')));
//             parse_passing!("'0'", Literal(Character('0')));
//             //TODO: Test escape sequences once implemented

//             parse_mismatch!("a", Literal(Character('a')));

//             parse_failing!("'String'");
//             parse_failing!("''");
//             parse_failing!("");
//         }

//         #[test]
//         fn string() {
//             parse_passing!("\"String\"", Literal(String("String".into())));
//             parse_passing!("\"\"", Literal(String("".into())));
//             //TODO: Test escape sequences once implemented

//             parse_mismatch!("abc", Literal(String("abc".into())));

//             parse_failing!("");
//         }

//         #[test]
//         fn boolean() {
//             parse_passing!("TRUE", Literal(Boolean(true)));
//             parse_passing!("FALSE", Literal(Boolean(false)));

//             parse_mismatch!("true", Literal(Boolean(true)));
//             parse_mismatch!("false", Literal(Boolean(false)));
//             parse_mismatch!("treu", Literal(Boolean(true)));

//             parse_failing!("");
//         }

//         #[test]
//         fn date() {
//             parse_passing!("`09/12/2023`", Literal(Date(date!(2023 - 12 - 09))));

//             parse_mismatch!("9/12/2023", Literal(Date(date!(2023 - 12 - 09))));

//             parse_failing!("`9/12/2023`");
//             parse_failing!("`09/1/2023`");
//             parse_failing!("`09/12/23`");
//             parse_failing!("");
//         }
//     }

//     #[test]
//     fn variable() {
//         parse_passing!("abc", Variable("abc".into()));
//         parse_passing!("abc_def", Variable("abc_def".into()));
//         parse_passing!("_abc", Variable("_abc".into()));
//         parse_passing!("_", Variable("_".into()));

//         parse_mismatch!("TRUE", Variable("TRUE".into()));
//         parse_mismatch!("123", Variable("123".into()));

//         parse_failing!("123abc");
//         parse_failing!("");
//     }

//     mod unary {
//         use super::*;

//         #[test]
//         fn not() {
//             parse_passing!("NOT TRUE", Not(Box::new(Literal(Boolean(true)))));
//             parse_passing!(
//                 "NOT NOT FALSE",
//                 Not(Box::new(Not(Box::new(Literal(Boolean(false))))))
//             );

//             parse_failing!("");
//         }

//         #[test]
//         fn negation() {
//             parse_passing!("-123", Negation(Box::new(Literal(Integer(123)))));
//             parse_passing!(
//                 "--123",
//                 Negation(Box::new(Negation(Box::new(Literal(Integer(123))))))
//             );

//             parse_failing!("");
//         }
//     }
// }
