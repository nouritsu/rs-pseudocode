use chumsky::Parser;
use clap::Parser as CLParser;
use color_eyre::{eyre, install as color_install, owo_colors::OwoColorize};
use humantime::format_duration;
use rs_pseudocode::{eval, parser};
use std::{
    collections::HashMap,
    fs,
    io::{self, Write},
    process::exit,
    time::{Duration, Instant},
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
        Some(f) => run_file(&f, args.time),
        None => run_repl(args.time),
    }

    Ok(())
}

fn run_file(f: &str, show_time: bool) {
    let src = fs::read_to_string(f).expect("unable to read source file");
    let (pt, et) = run(&src);
    if show_time {
        println!(
            "Parse Time    :   {}",
            format_duration(pt).to_string().bright_blue()
        );
        println!(
            "Exec Time     :   {}",
            format_duration(et).to_string().bright_red()
        );
    }
}

fn run_repl(show_time: bool) {
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

        let (pt, et) = run(&line);
        if show_time {
            println!(
                "Parse Time    :   {}",
                format_duration(pt).to_string().bright_blue()
            );
            println!(
                "Exec Time     :   {}",
                format_duration(et).to_string().bright_red()
            );
        }
    }
}

fn run(src: &str) -> (Duration, Duration) {
    let mut pt = Duration::default();
    let mut et = Duration::default();

    let mut reference = Instant::now();
    match parser().parse(src) {
        Ok(res) => {
            pt = Instant::now() - reference;
            reference = Instant::now();
            match eval(&res, &mut HashMap::new()) {
                Ok(res) => {
                    et = Instant::now() - reference;
                    println!("Result        :   {}", res.bright_black())
                }
                Err(_) => todo!("error handling"),
            }
        }
        Err(err) => err.into_iter().for_each(|e| println!("{}", e)),
    };

    return (pt, et);
}
