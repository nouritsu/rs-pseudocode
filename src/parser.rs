use pest_derive::Parser;

macro_rules! assert_rule_ok {
    ($rule: expr, $src: literal) => {
        assert!(PseudocodeParser::parse($rule, $src).is_ok());
    };
}

macro_rules! assert_rule_err {
    ($rule: expr, $src: literal) => {
        assert!(PseudocodeParser::parse($rule, $src).is_err());
    };
}

#[derive(Parser)]
#[grammar = "grammar.pest"]
pub struct PseudocodeParser;

#[cfg(test)]
mod tests {
    use super::*;
    pub use pest::Parser;

    mod literals {
        use super::*;

        #[test]
        fn integer() {
            assert_rule_ok!(Rule::LInteger, "0123456789");
            assert_rule_err!(Rule::LInteger, "Hello");
            assert_rule_err!(Rule::LInteger, "");
        }

        #[test]
        fn real() {
            assert_rule_ok!(Rule::LReal, "01234.56789");
            assert_rule_err!(Rule::LReal, "420");
            assert_rule_err!(Rule::LReal, "Hello");
            assert_rule_err!(Rule::LReal, "");
        }

        #[test]
        fn boolean() {
            assert_rule_ok!(Rule::LBoolean, "TRUE");
            assert_rule_ok!(Rule::LBoolean, "FALSE");
            assert_rule_err!(Rule::LBoolean, "Hello");
            assert_rule_err!(Rule::LBoolean, "");
        }

        #[test]
        fn char() {
            assert_rule_ok!(Rule::LChar, "'h'");
            assert_rule_ok!(Rule::LChar, "'1'");
            assert_rule_ok!(Rule::LChar, "'\"'");
            assert_rule_err!(Rule::LChar, "'Hello'");
            assert_rule_err!(Rule::LChar, "''"); // Empty characters are invalid
            assert_rule_err!(Rule::LChar, "");
        }

        #[test]
        fn string() {
            assert_rule_ok!(Rule::LString, "\"Hello\"");
            assert_rule_ok!(Rule::LString, "\"123.123\"");
            assert_rule_ok!(Rule::LString, "\"\"\\\t\n\r\"");
            assert_rule_ok!(Rule::LString, "\"\"");
            assert_rule_err!(Rule::LString, "Hello");
            assert_rule_err!(Rule::LString, "");
        }
    }

    #[test]
    fn data_type() {
        assert_rule_ok!(Rule::DataType, "INTEGER");
        assert_rule_ok!(Rule::DataType, "REAL");
        assert_rule_ok!(Rule::DataType, "BOOLEAN");
        assert_rule_ok!(Rule::DataType, "CHAR");
        assert_rule_ok!(Rule::DataType, "STRING");
        assert_rule_err!(Rule::DataType, "123STRING");
        assert_rule_err!(Rule::DataType, "");
    }

    #[test]
    fn identifier() {
        assert_rule_ok!(Rule::Identifier, "hello_world");
        assert_rule_ok!(Rule::Identifier, "_unused");
        assert_rule_err!(Rule::Identifier, "123hello");
        assert_rule_err!(Rule::Identifier, "");
    }

    mod exprs {
        use super::*;

        #[test]
        fn literal() {
            assert_rule_ok!(Rule::LiteralExpr, "123");
            assert_rule_ok!(Rule::LiteralExpr, "123.456");
            assert_rule_ok!(Rule::LiteralExpr, "'x'");
            assert_rule_ok!(Rule::LiteralExpr, "\"xyz\"");
            assert_rule_ok!(Rule::LiteralExpr, "TRUE");
            assert_rule_err!(Rule::LiteralExpr, "some_identifier");
            assert_rule_err!(Rule::LiteralExpr, "");
        }

        #[test]
        fn unary() {
            assert_rule_ok!(Rule::UnaryExpr, "-123");
            assert_rule_ok!(Rule::UnaryExpr, "+ 123");
            assert_rule_ok!(Rule::UnaryExpr, "-\"HELLO\""); // Syntactically correct, error at runtime
            assert_rule_ok!(Rule::UnaryExpr, "-some_identifier");
            assert_rule_err!(Rule::UnaryExpr, "123 + 123");
            assert_rule_err!(Rule::UnaryExpr, "");
        }

        #[test]
        fn binary() {
            assert_rule_ok!(Rule::BinaryExpr, "123 + 123");
            assert_rule_ok!(Rule::BinaryExpr, "'a'-'b'");
            assert_rule_ok!(Rule::BinaryExpr, "\"Hello\"-\"There\""); // Syntactically correct, error at runtime
            assert_rule_ok!(Rule::BinaryExpr, "some_identifier    / some_identifier");
            assert_rule_err!(Rule::BinaryExpr, "-123");
            assert_rule_err!(Rule::BinaryExpr, "");
        }

        #[test]
        fn logical() {
            assert_rule_ok!(Rule::LogicalExpr, "123 AND 123");
            assert_rule_ok!(Rule::LogicalExpr, "'a' OR 'b'");
            assert_rule_ok!(Rule::LogicalExpr, "some_identifier AND some_identifier");
            assert_rule_err!(Rule::LogicalExpr, "AND 123");
            assert_rule_err!(Rule::LogicalExpr, "");
        }

        #[test]
        fn grouping() {
            assert_rule_ok!(Rule::GroupingExpr, "(\"Hello\")");
            assert_rule_err!(Rule::GroupingExpr, "(1 + 2)");
            assert_rule_err!(Rule::GroupingExpr, "(TRUE AND TRUE)");
            assert_rule_err!(Rule::GroupingExpr, "(1 + 2");
            assert_rule_err!(Rule::GroupingExpr, "1 + 2)");
            assert_rule_err!(Rule::GroupingExpr, "");
        }
    }
}
