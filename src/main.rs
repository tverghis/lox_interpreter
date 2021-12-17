use std::io::{self, BufRead, Write};

use anyhow::{Context, Result};

use clap::{App, Arg};
use lox_interpreter::Scanner;

fn main() -> Result<()> {
    let parsed_args = App::new("lox")
        .version("0.1.0")
        .author("Tarun Verghis <tarun.verghis@gmail.com>")
        .about("Lox interpreter")
        .arg(
            Arg::with_name("file")
                .short("f")
                .long("file")
                .value_name("FILE")
                .help("Name of the Lox source file to execute")
                .takes_value(true)
                .required(false),
        )
        .get_matches();

    match parsed_args.value_of("file") {
        Some(file) => run_file(file),
        None => run_prompt(),
    }
}

// Runs a Lox source file
fn run_file(file_path: &str) -> Result<()> {
    let source =
        std::fs::read_to_string(file_path).context(format!("Could not run `{}`", file_path))?;

    run(&source)?;

    Ok(())
}

// Runs an interactive REPL that can interpret and execute Lox statements
fn run_prompt() -> Result<()> {
    let stdin = io::stdin();
    let mut stdin_handle = stdin.lock();

    let mut input = String::new();

    loop {
        input.clear();

        print!("> ");
        io::stdout().flush()?;

        stdin_handle.read_line(&mut input)?;

        if input.is_empty() {
            // Ensure a newline is printed when the prompt is interrupted
            println!();
            break;
        }

        let result = run(&input);

        if let Err(e) = result {
            println!("Error: {}.", e);
        }
    }

    Ok(())
}

// Executes the provided Lox source
fn run(source: &str) -> Result<()> {
    let tokens = Scanner::new(source).scan_tokens()?;

    for token in tokens {
        dbg!(token);
    }

    Ok(())
}
