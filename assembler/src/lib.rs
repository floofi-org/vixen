pub mod compiler;
pub mod scanner;
pub mod models;
pub mod error;
pub mod parser;
pub mod preprocessor;

use error::Error;

pub fn assemble(source: String) -> Result<Vec<u8>, Error> {
    let tokens = scanner::Scanner::new(&source).scan()?;
    let program = parser::Parser::new(tokens).parse()?;
    let instructions = preprocessor::Preprocessor::process(program)?;
    let compiled = compiler::Compiler::default().compile(instructions)?;

    Ok(compiled)
}
