use std::collections::{HashMap, VecDeque};

use vixen::core::instruction::Operation;

use crate::models::token::Literal;
use crate::models::{Instruction, Token};

use super::label::Label;
use super::operation::OperationExt;
use super::r#macro::Macro;
use super::{FromTokenStream, ParseError, Parser};

#[derive(Debug, Default)]
pub struct Program {
    pub labels: HashMap<String, usize>,
    pub macros: Vec<(Macro, usize)>,
    pub instructions: VecDeque<Instruction>,
}

impl FromTokenStream for Program {
    fn parse(parser: &mut Parser) -> Result<Self, ParseError> {
        let mut labels = HashMap::new();
        let mut macros = Vec::new();
        let mut instructions = VecDeque::new();

        loop {
            let token = parser.peek()?;

            match token {
                Token::Dot => {
                    let r#macro = Macro::parse(parser)?;
                    macros.push((r#macro, instructions.len()))
                }

                Token::Literal(Literal::Identifier(ident)) => {
                    identifier(ident.clone(), &mut labels, &mut instructions, parser)?;
                }
                Token::LineBreak => {
                    parser.next().unwrap();
                }
                Token::EOF => break,
                t => return Err(ParseError::UnexpectedToken(t.clone())),
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
    labels: &mut HashMap<String, usize>,
    instructions: &mut VecDeque<Instruction>,
    parser: &mut Parser,
) -> Result<(), ParseError> {
    parser.next();
    let next = parser.peek()?;

    if let Token::Colon = next {
        let label = Label::parse(identifier, parser)?;

        labels.insert(label.0, instructions.len());
    } else {
        let operation = Operation::parse(identifier)?;
        let instruction = Instruction::parse(operation, parser)?;

        instructions.push_back(instruction);
    }

    Ok(())
}
