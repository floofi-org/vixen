use vixen::core::instruction::Operation;

use crate::models::token::Literal;
use crate::models::Token;

use super::instruction::Instruction;
use super::label::Label;
use super::operation::OperationExt;
use super::r#macro::Macro;
use super::{FromTokenStream, ParseError, Parser};

#[derive(Debug, Default)]
pub struct Program {
    pub labels: Vec<Label>,
    pub macros: Vec<Macro>,
    pub instructions: Vec<Instruction>,
}

impl FromTokenStream for Program {
    fn parse(parser: &mut Parser) -> Result<Self, ParseError> {
        let mut labels = Vec::new();
        let mut macros = Vec::new();
        let mut instructions = Vec::new();

        loop {
            let token = parser.peek()?;

            match token {
                Token::Dot => {
                    let r#macro = Macro::parse(parser)?;
                    macros.push(r#macro);
                }

                Token::Literal(Literal::Identifier(ident)) => {
                    identifier(ident.clone(), &mut labels, &mut instructions, parser)?;
                }
                Token::LineBreak => {}
                Token::EOF => break,
                _ => return Err(ParseError::UnexpectedToken),
            }
        }

        Ok(Self {
            labels,
            macros,
            instructions,
        })
    }
}

fn identifier(
    identifier: String,
    labels: &mut Vec<Label>,
    instructions: &mut Vec<Instruction>,
    parser: &mut Parser,
) -> Result<(), ParseError> {
    parser.next();
    let next = parser.peek()?;

    if let Token::Colon = next {
        let label = Label::parse(identifier, parser)?;
        labels.push(label);
    } else {
        let operation = Operation::parse(identifier)?;
        let instruction = Instruction::parse(operation, parser)?;
        instructions.push(instruction);
    }

    Ok(())
}
