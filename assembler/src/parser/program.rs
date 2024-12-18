use std::collections::{HashMap, VecDeque};

use vixen::core::instruction::Operation;

use crate::models::token::Literal;
use crate::models::{Instruction, Token};

use super::label::Label;
use super::operation::OperationExt;
use super::r#macro::MacroDefinition;
use super::{Constant, FromTokenStream, ParseError, Parser};

#[derive(Debug, Default)]
pub struct Program {
    pub constants: HashMap<String, u32>,
    pub labels: HashMap<String, usize>,
    pub macros: Vec<(MacroDefinition, usize)>,
    pub instructions: VecDeque<Instruction>,
}

impl FromTokenStream for Program {
    fn parse(parser: &mut Parser) -> Result<Self, ParseError> {
        let mut constants = HashMap::new();
        let mut labels = HashMap::new();
        let mut macros = Vec::new();
        let mut instructions = VecDeque::new();

        loop {
            let token = parser.peek()?;

            match token {
                Token::Dot => {
                    let r#macro = MacroDefinition::parse(parser)?;
                    macros.push((r#macro, instructions.len()));
                },

                Token::Literal(Literal::Identifier(ident)) => {
                    identifier(ident.clone(), &mut constants, &mut labels, &mut instructions, parser)?;
                },

                Token::LineBreak => {
                    parser.next().unwrap();
                },

                Token::EOF => break,
                t => return Err(ParseError::UnexpectedToken(t.clone())),
            }
        }

        Ok(Self {
            constants,
            labels,
            macros,
            instructions,
        })
    }
}

fn identifier(
    identifier: String,
    constants: &mut HashMap<String, u32>,
    labels: &mut HashMap<String, usize>,
    instructions: &mut VecDeque<Instruction>,
    parser: &mut Parser,
) -> Result<(), ParseError> {
    parser.next().unwrap();
    let next = parser.peek()?;

    match next {
        Token::Colon => {
            let label = Label::parse(identifier, parser)?;
            labels.insert(label.0, instructions.len());
        },

        Token::Equals => {
            let constant = Constant::parse(identifier, parser)?;

            if constants.contains_key(&constant.0) {
                return Err(ParseError::ConstantAlreadyDefined(constant.0));
            }

            constants.insert(constant.0, constant.1);
        }

        _ => {
            let operation = Operation::parse(identifier)?;
            let instruction = Instruction::parse(operation, parser)?;

            instructions.push_back(instruction);
        }
    }

    Ok(())
}
