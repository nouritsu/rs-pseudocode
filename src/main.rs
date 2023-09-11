use crate::parser::{PseudocodeParser, Rule};
use pest::Parser;

mod parser;

fn main() {
    let r = PseudocodeParser::parse(Rule::GroupingExpr, "(-1)");
    println!("{:?}", r);
}
