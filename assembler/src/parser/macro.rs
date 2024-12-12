use crate::models::token::{Dot, Identifier};

use super::{FromTokenStream, ParseError, Parser};

#[derive(Debug)]
pub struct Macro {
    name: String,
}

impl FromTokenStream for Macro {
    fn parse(parser: &mut Parser) -> Result<Self, ParseError> {
        parser.expect::<Dot>()?;

        let name: Identifier = parser.expect()?;
        let name = name.0;

        Ok(Self {
            name,
        })
    }
}
