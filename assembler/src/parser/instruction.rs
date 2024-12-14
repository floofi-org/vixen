use vixen::core::instruction::Operation;

use crate::models::{Instruction, Operand};

use super::{FromTokenStream, ParseError, Parser};
use super::args::Args;

// FIXME: See label module
// impl FromTokenStream for Instruction {}

impl Instruction {
    pub fn parse(operation: Operation, parser: &mut Parser) -> Result<Self, ParseError> {
        let operands = Args::for_each(parser, Operand::parse)?;

        Ok(Self {
            operation,
            operands,
        })
    }
}
