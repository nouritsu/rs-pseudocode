use crate::parser::{PseudocodeParser, Rule};
use pest::Parser;

mod parser;

fn main() {
    let r = PseudocodeParser::parse(Rule::UnaryExpr, "-Hello");
    println!("{:?}", r)
}
