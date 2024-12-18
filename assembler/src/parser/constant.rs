use crate::models::token::{Equals, Number};

use super::{ParseError, Parser};

pub struct Constant(pub String, pub u32);

impl Constant {
    pub fn parse(name: String, parser: &mut Parser) -> Result<Self, ParseError> {
        parser.expect::<Equals>()?;

        let value: Number = parser.expect()?;

        Ok(Self(name, value.0))
    }
}
