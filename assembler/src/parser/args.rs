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
        while let Some(Token::Comma) = parser.next_on_line()? {
            let arg = f(parser)?;
            args.push(arg);
        }

        Ok(args)
    }
}
