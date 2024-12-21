use crate::models::token::{Dot, Identifier, StringLiteral};

use super::{args::Args, FromTokenStream, ParseError, Parser};

#[derive(Debug)]
#[allow(clippy::module_name_repetitions)]
pub struct MacroDefinition {
    pub name: String,
    pub args: Vec<MacroArg>,
}

#[allow(clippy::module_name_repetitions)]
#[derive(Debug)]
pub struct MacroArg(pub String);

impl FromTokenStream for MacroArg {
    fn parse(parser: &mut Parser) -> Result<Self, ParseError> {
        let arg = parser.expect::<StringLiteral>()?;

        Ok(Self(arg.0))
    }
}

impl FromTokenStream for MacroDefinition {
    fn parse(parser: &mut Parser) -> Result<Self, ParseError> {
        parser.expect::<Dot>()?;

        let name: Identifier = parser.expect()?;
        let name = name.0;

        let args = Args::for_each(parser, MacroArg::parse)?;

        Ok(Self {
            name,
            args,
        })
    }
}
