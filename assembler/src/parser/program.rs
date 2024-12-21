use std::collections::{HashMap, VecDeque};

use vixen::core::instruction::Operation;

use crate::models::token::Literal;
use crate::models::{Instruction, Token};

use super::label::Label;
use super::operation::OperationExt;
use super::r#macro::MacroDefinition;
use super::{Constant, FromTokenStream, ParseError, Parser};

const BUILTIN_CONSTANTS: &[(&str, u32)] = &[];

#[derive(Debug)]
pub struct Program {
    pub constants: HashMap<String, u32>,
    pub labels: HashMap<String, usize>,
    pub macros: Vec<(MacroDefinition, usize)>,
    pub instructions: VecDeque<Instruction>,
}

impl Default for Program {
    fn default() -> Self {
        let constants = BUILTIN_CONSTANTS.iter()
            .map(|(k, v)| ((*k).to_owned(), *v))
            .collect();

        Self {
            constants,
            labels: HashMap::default(),
            macros: Vec::default(),
            instructions: VecDeque::default(),
        }
    }
}


impl FromTokenStream for Program {
    fn parse(parser: &mut Parser) -> Result<Self, ParseError> {
        let mut program = Program::default();

        loop {
            let token = parser.peek()?;

            match token {
                Token::Dot => {
                    let r#macro = MacroDefinition::parse(parser)?;
                    program.macros.push((r#macro, program.instructions.len()));
                },

                Token::Literal(Literal::Identifier(ident)) => {
                    identifier(ident.clone(), &mut program.constants, &mut program.labels, &mut program.instructions, parser)?;
                },

                Token::LineBreak => {
                    parser.next().unwrap();
                },

                Token::EOF => break,
                t => return Err(ParseError::UnexpectedToken(t.clone())),
            }
        }

        Ok(program)
    }
}

fn identifier(
    identifier: String,
    constants: &mut HashMap<String, u32>,
    labels: &mut HashMap<String, usize>,
    instructions: &mut VecDeque<Instruction>,
    parser: &mut Parser,
) -> Result<(), ParseError> {
    parser.next()?;
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
