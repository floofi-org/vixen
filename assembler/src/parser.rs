use std::vec::IntoIter;
use std::iter::Peekable;

use crate::models::{Token, TokenWithSpan};
use crate::models::token::FromToken;

mod args;
mod instruction;
mod label;
mod r#macro;
mod operand;
mod operation;
mod program;

pub use label::Label;
pub use r#macro::{MacroDefinition, MacroArg};
pub use program::Program;

trait FromTokenStream: Sized {
    fn parse(parser: &mut Parser) -> Result<Self, ParseError>;
}

#[derive(Debug)]
pub enum ParseError {
    UnexpectedToken(Token),
    UnexpectedEof,
    InvalidInstruction(String),
    InvalidOperand(&'static str)
}

pub struct Parser {
    tokens: Peekable<IntoIter<TokenWithSpan>>,
}

impl Parser {
    #[must_use]
    pub fn new(tokens: Vec<TokenWithSpan>) -> Self {
        let tokens = tokens.into_iter().peekable();

        Self {
            tokens,
        }
    }

    pub fn parse(&mut self) -> Result<Program, ParseError> {
        Program::parse(self)
    }

    fn expect<T: FromToken>(&mut self) -> Result<T, ParseError> {
        let Some(token) = self.tokens.next() else {
            return Err(ParseError::UnexpectedEof);
        };

        let Some(expected) = T::from_token(token.token.clone()) else {
            return Err(ParseError::UnexpectedToken(token.token));
        };

        Ok(expected)
    }

    fn next(&mut self) -> Result<Token, ParseError> {
        let Some(token) = self.tokens.next() else {
            return Err(ParseError::UnexpectedEof);
        };

        Ok(token.token)
    }

    fn peek(&mut self) -> Result<&Token, ParseError> {
        let Some(token) = self.tokens.peek() else {
            return Err(ParseError::UnexpectedEof);
        };

        Ok(&token.token)
    }

    fn next_on_line(&mut self) -> Result<Option<Token>, ParseError> {
        let Some(token) = self.tokens.next() else {
            return Err(ParseError::UnexpectedEof);
        };

        if let Token::LineBreak | Token::EOF = token.token {
            Ok(None)
        } else {
            Ok(Some(token.token))
        }
    }
}
