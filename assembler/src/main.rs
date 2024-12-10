use std::fs;
use std::path::PathBuf;
use std::process::exit;
use vasm::scanner::Scanner;
use clap::Parser;
use vasm::error::Result;

/// A code assembler for Vixen processors
#[derive(Parser, Debug)]
#[command(about)]
struct Args {
    pub source: PathBuf,
}

fn run_assembler(args: Args) -> Result<()> {
    let source = fs::read_to_string(args.source)?;
    let tokens = Scanner::new(&source).scan();
    println!("{tokens:#?}");

    let mut parser = vasm::parser::Parser::new(tokens);
    let program = parser.parse().unwrap();

    println!("{program:#?}");
    Ok(())
}

fn main() {
    let args = Args::parse();
    if let Err(e) = run_assembler(args) {
        eprintln!("error: {e}");
        exit(1);
    }
}
