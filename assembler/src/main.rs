use std::fs;
use std::path::PathBuf;
use std::process::exit;
use vasm::assemble;
use clap::Parser;
use vasm::error::Result;

/// A code assembler for Vixen processors
#[derive(Parser, Debug)]
#[command(about)]
struct Args {
    pub source: PathBuf,
    pub destination: PathBuf,
}

fn run_assembler(args: &Args) -> Result<()> {
    let source = fs::read_to_string(&args.source)?;
    let program = assemble(&args.source, &source)?;
    fs::write(&args.destination, program)?;

    println!("Compiled program {} to {}", args.source.display(), args.destination.display());
    Ok(())
}

fn main() {
    let args = Args::parse();
    if let Err(e) = run_assembler(&args) {
        eprintln!("error: {e}");
        exit(1);
    }
}
