use chumsky::Parser;
use clap::Parser as CLParser;
use color_eyre::{eyre, install as color_install};
use rs_pseudocode::parser::parser;
use std::{
    fs,
    io::{self, Write},
    process::exit,
};

#[derive(CLParser)]
struct Args {
    /// Source File
    src: Option<String>,

    /// Show time taken for each step
    #[arg(short, long)]
    time: bool,
}

fn main() -> eyre::Result<()> {
    color_install()?;

    let args = Args::parse();

    match args.src {
        Some(f) => run_file(&f),
        None => run_repl(),
    }

    Ok(())
}

fn run_file(f: &str) {
    let src = fs::read_to_string(f).expect("unable to read source file");
    run(&src);
}

fn run_repl() {
    loop {
        print!(":> ");
        io::stdout().flush().expect("unable to flush stdout");

        let mut line = String::new();
        io::stdin()
            .read_line(&mut line)
            .expect("unable to read from stdin");

        if line.starts_with("exit") || line.starts_with("quit") {
            exit(0);
        }

        run(&line);
    }
}

fn run(src: &str) {
    match parser().parse(src) {
        Ok(res) => println!("{}", res),
        Err(err) => err.into_iter().for_each(|e| println!("{}", e)),
    }
}
