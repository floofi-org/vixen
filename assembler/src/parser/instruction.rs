use vixen::core::instruction::Operation;

use crate::models::{Instruction, Operand, Token};

use super::{FromTokenStream, ParseError, Parser};

// FIXME: See label module
// impl FromTokenStream for Instruction {}

impl Instruction {
    pub fn parse(operation: Operation, parser: &mut Parser) -> Result<Self, ParseError> {
        let mut operands = Vec::new();

        // First arg
        match parser.peek()? {
            Token::LineBreak | Token::EOF => {}
            _ => {
                operand(&mut operands, parser)?;
            }
        }

        // Others expect comma
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