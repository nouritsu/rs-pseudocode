mod expr;
mod parser;
mod value;

use chumsky::Parser;
use color_eyre::{eyre, install as color_install};
use parser::parser;
use std::{env, fs};

fn main() -> eyre::Result<()> {
    color_install()?;

    let src = fs::read_to_string(env::args().nth(1).expect("pass file name as arg 1"))
        .expect("unable to read file");

    match parser().parse(src) {
        Ok(res) => println!("Successful, Result: {}", res),
        Err(err) => err.into_iter().for_each(|e| println!("Parse Error: {}", e)),
    }

    Ok(())
}
