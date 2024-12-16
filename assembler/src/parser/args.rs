use crate::models::Token;

use super::{ParseError, Parser};

pub struct Args;

impl Args {
    pub fn for_each<T, F>(parser: &mut Parser, mut f: F) -> Result<Vec<T>, ParseError>
    where
        F: FnMut(&mut Parser) -> Result<T, ParseError>
    {
        let mut args = Vec::new();

        // First arg
        match parser.peek()? {
            Token::LineBreak | Token::EOF => {},
            _ => {
                let arg = f(parser)?;
                args.push(arg);
            }
        }

        // Others expect comma
        while let Some(token) = parser.next_on_line()? {
            match token {
                Token::Comma => {
                    let arg = f(parser)?;
                    args.push(arg);
                }
                t => return Err(ParseError::UnexpectedToken(t)),
            }
        }

        Ok(args)
    }
}
