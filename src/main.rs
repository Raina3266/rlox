use std::{
    error::Error,
    io::{BufRead, BufReader, Write, stdin, stdout},
    path::Path,
};

use crate::{
    error::{had_error, reset_error},
    lexer::Scanner,
    parser::Parser,
};

mod error;
mod interpreter;
mod lexer;
mod parser;

fn main() -> Result<(), Box<dyn Error>> {
    let mut args = std::env::args().skip(1);
    if args.len() > 1 {
        eprintln!("Usage: rlox [SCRIPT]");
        std::process::exit(64);
    } else if args.len() == 1 {
        run_file(Path::new(&args.next().unwrap()))?;
    } else {
        run_prompt()?;
    }
    Ok(())
}

fn run_file(path: &Path) -> Result<(), Box<dyn Error>> {
    run(&std::fs::read_to_string(path)?);
    if had_error() {
        std::process::exit(65);
    }
    Ok(())
}

fn run_prompt() -> Result<(), Box<dyn Error>> {
    let input = stdin();
    let mut input = BufReader::new(input);
    loop {
        print!("> ");
        stdout().flush().unwrap();
        let mut line = String::new();
        input.read_line(&mut line).unwrap();
        run(&line)?;
        reset_error();
    }
}

fn run(line: &str) -> Result<(), Box<dyn Error>> {
    let mut scanner = Scanner::new(line);
    let tokens = scanner.scan_tokens();
    let mut parser = Parser::new(tokens);
    let expression = parser.parse();
    if had_error() {
        return Ok(());
    }
    if let Ok(expression) = expression {
        println!("{}", expression.pretty_print());
    }
    Ok(())
}
