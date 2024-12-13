use std::fmt::{Display, Formatter};
use std::io;

use crate::compiler::CompilerError;
use crate::parser::ParseError;
use crate::preprocessor::PreprocessorError;
use crate::scanner::UnexpectedToken;

pub type Result<T> = core::result::Result<T, Error>;

pub enum Error {
    IO(io::Error),
    UnexpectedToken(char),
    ParserError(ParseError),
    PreprocessorError(PreprocessorError),
    CompilerError(CompilerError),
}

impl From<io::Error> for Error {
    fn from(value: io::Error) -> Self {
        Self::IO(value)
    }
}

impl From<UnexpectedToken> for Error {
    fn from(value: UnexpectedToken) -> Self {
        Self::UnexpectedToken(value.0)
    }
}

impl From<ParseError> for Error {
    fn from(value: ParseError) -> Self {
        Self::ParserError(value)
    }
}

impl From<PreprocessorError> for Error {
    fn from(value: PreprocessorError) -> Self {
        Self::PreprocessorError(value)
    }
}

impl From<CompilerError> for Error {
    fn from(value: CompilerError) -> Self {
        Self::CompilerError(value)
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::IO(e) => write!(f, "Input/output error: {e}"),
            Self::UnexpectedToken(c) => write!(f, "Encountered unexpected token: {c}"),
            Self::ParserError(e) => write!(f, "Parser error: {e:?}"),
            Self::PreprocessorError(e) => write!(f, "Preprocessor error: {e:?}"),
            Self::CompilerError(e) => write!(f, "Compiler error: {e:?}"),
        }
    }
}
