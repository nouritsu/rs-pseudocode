use chumsky::Parser;
use clap::Parser as CLParser;
use color_eyre::{eyre, install as color_install};
use rs_pseudocode::{exec, parser};
use std::{
    collections::HashMap,
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

    #[arg(short, long)]
    exec: Option<String>,
}

fn main() -> eyre::Result<()> {
    color_install()?;

    let args = Args::parse();
    match args.src {
        Some(f) => run_file(&f),
        None => match args.exec {
            Some(src) => run(&src),
            None => run_repl(),
        },
    }

    Ok(())
}

fn run_file(f: &str) {
    let src = fs::read_to_string(f).expect("unable to read source file");
    run(&src);
}

fn run_repl() {
    loop {
        let mut line = String::new();

        print!(":> ");
        io::stdout().flush().expect("unable to flush stdout");

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
    match parser().parse(src).into_result() {
        Ok(ast) => {
            for stmt in ast.iter() {
                match exec(stmt, &mut HashMap::new()) {
                    Ok(Some(x)) => println!("{}", x),
                    Err(e) => println!("Exec Error: {:?}", e),
                    _ => {}
                }
            }
        }
        Err(parse_err) => parse_err
            .into_iter()
            .for_each(|e| println!("Parse Error: {:?}", e)),
    };
}
