use vixen::core::instruction::Operation;

use crate::models::Token;

use super::{FromTokenStream, ParseError, Parser};
use super::operand::Operand;

#[derive(Debug)]
pub struct Instruction {
    operation: Operation,
    operands: Vec<Operand>,
}

// FIXME: See label module
// impl FromTokenStream for Instruction {}

impl Instruction {
    pub fn parse(operation: Operation, parser: &mut Parser) -> Result<Self, ParseError> {
        let mut operands = Vec::new();

        operand(&mut operands, parser)?;
        while let Some(Token::Comma) = parser.next_on_line()? {
            operand(&mut operands, parser)?;
        }

        Ok(Self {
            operation,
            operands,
        })
    }
}

fn operand(operands: &mut Vec<Operand>, parser: &mut Parser) -> Result<(), ParseError> {
    let operand = Operand::parse(parser)?;
    operands.push(operand);

    Ok(())
}
