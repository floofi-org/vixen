use std::cmp::PartialEq;
use vixen::core::registers::register_id::RegisterId;
use crate::models::{Span, Token, TokenWithSpan};

#[derive(Debug, PartialEq, Clone)]
pub enum Operand {
    None,
    Literal(u16),
    Address(u16),
    Relative(i16),
    Register(RegisterId),
    Label(String)
}

#[derive(Debug)]
pub enum ParsedInstructionData {
    Label(String),
    Instruction(String, Operand, Operand),
    Macro(String, String)
}

#[derive(Debug)]
pub struct ParsedInstruction {
    pub instruction: ParsedInstructionData,
    pub span: Span
}

#[derive(Debug)]
pub struct ParsedCollection(pub Vec<ParsedInstruction>);

impl From<Vec<TokenWithSpan>> for ParsedCollection {
    fn from(value: Vec<TokenWithSpan>) -> Self {
        let mut collection = ParsedCollection(vec![]);
        let mut iter = value.iter().peekable();
        let mut current_instruction = (String::new(), Operand::None, Operand::None);
        let mut current_macro = (String::new(), String::new());
        let mut literal = false;
        let mut signed = false;
        let mut negative = false;
        let mut parsing_macro = false;

        while let Some(token) = iter.next() {
            println!("{:?} {:?}", &token.token, current_instruction);

            let current_operand = if current_instruction.1 == Operand::None {
                &mut current_instruction.1
            } else {
                &mut current_instruction.2
            };

            match &token.token {
                Token::Identifier(i) => {
                    if parsing_macro {
                        if current_macro.0.is_empty() {
                            i.clone_into(&mut current_macro.0);
                        } else {
                            i.clone_into(&mut current_macro.1);
                        }
                    } else if let Some(TokenWithSpan { token: Token::Colon, .. }) = iter.peek() {
                        collection.0.push(ParsedInstruction {
                            instruction: ParsedInstructionData::Label(i.to_owned()),
                            span: token.span.clone()
                        });
                    } else if current_instruction.0.is_empty() {
                        current_instruction.0 = i.to_string();
                    } else if let Ok(id) = i.try_into() {
                        *current_operand = Operand::Register(id);
                    } else {
                        *current_operand = Operand::Label(i.to_string());
                    }
                },
                Token::LineBreak | Token::EOF => {
                    if parsing_macro {
                        collection.0.push(ParsedInstruction {
                            instruction: ParsedInstructionData::Macro(
                                current_macro.0.clone(), current_macro.1.clone()
                            ),
                            span: token.span.clone()
                        });
                        current_macro = (String::new(), String::new());
                        parsing_macro = false;
                    } else if !current_instruction.0.is_empty() {
                        collection.0.push(ParsedInstruction {
                            instruction: ParsedInstructionData::Instruction(
                                current_instruction.0.clone(), current_instruction.1.clone(), current_instruction.2.clone()
                            ),
                            span: token.span.clone()
                        });
                        current_instruction = (String::new(), Operand::None, Operand::None);
                    }
                },
                Token::Dot => {
                    parsing_macro = true;
                }
                Token::Hash => {
                    literal = true;
                },
                Token::Plus => {
                    signed = true;
                    negative = false;
                },
                Token::Minus => {
                    signed = true;
                    negative = true;
                },
                Token::Number(n) => {
                    // We consider that a sane user wouldn't enter an invalid number
                    #[allow(clippy::cast_possible_wrap)]
                    if literal {
                        *current_operand = Operand::Literal(*n);
                    } else if signed {
                        *current_operand = Operand::Relative(if negative {
                            -(*n as i16)
                        } else {
                            *n as i16
                        });
                    } else {
                        *current_operand = Operand::Address(*n);
                    }
                },
                Token::Colon | Token::Comma => {}
            }
        }

        collection
    }
}
