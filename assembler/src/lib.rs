pub mod compiler;
pub mod scanner;
pub mod models;
pub mod error;
pub mod parser;
pub mod preprocessor;

use std::path::Path;

use error::Error;
use preprocessor::ProcessedProgram;

pub fn assemble(source_path: &Path, source: &str) -> Result<Vec<u8>, Error> {
    let tokens = scanner::Scanner::new(source).scan()?;
    let program = parser::Parser::new(tokens).parse()?;
    let program = preprocessor::Preprocessor::process(source_path, program, false)?;
    let compiled = compiler::Compiler::default().compile(program.instructions)?;

    Ok(compiled)
}

pub fn compile_for_include(source_path: &Path, source: &str) -> Result<ProcessedProgram, Error> {
    let tokens = scanner::Scanner::new(source).scan()?;
    let program = parser::Parser::new(tokens).parse()?;
    let program = preprocessor::Preprocessor::process(source_path, program, true)?;

    Ok(program)
}
