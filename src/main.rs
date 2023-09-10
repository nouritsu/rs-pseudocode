use crate::parser::{PseudocodeParser, Rule};
use pest::Parser;

mod parser;

fn main() {
    let r = PseudocodeParser::parse(Rule::LReal, ".0");
    println!("{:?}", r)
}
