use crate::models::token::Colon;

use super::{ParseError, Parser};

#[derive(Debug)]
pub struct Label(pub String);

// NOTE: Label is a special case as we cannot know whether it's an operation or label
// by just looking at the first token.
// Parser doesn't support look ahead yet.
//
// main:
// ^^^^
// main 1, 2
// ^^^^
// impl FromTokenStream for Label {
//     fn parse(parser: &mut Parser) -> Result<Self, ParseError> {
//         let name: Identifier = parser.expect()?;
//         parser.expect::<Colon>()?;

//         Ok(Self(name.0))
//     }
// }

impl Label {
    pub fn parse(name: String, parser: &mut Parser) -> Result<Self, ParseError> {
        parser.expect::<Colon>()?;

        Ok(Self(name))
    }
}
