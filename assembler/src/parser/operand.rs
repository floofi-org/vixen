use vixen::core::registers::RegisterId;

use crate::models::operand::OperandIndirect;
use crate::models::{Address, Operand, Token};
use crate::models::token::{Identifier, Literal, Number, RightBracket, RightCurlyBracket};

use super::{FromTokenStream, ParseError, Parser};

impl FromTokenStream for Operand {
    fn parse(parser: &mut Parser) -> Result<Self, ParseError> {
        let token = parser.next()?;

        match token {
            Token::Hash => literal(parser),
            Token::LeftBracket => indirect(parser),
            Token::LeftCurlyBracket => constant(parser, false),
            Token::Literal(Literal::Identifier(register)) => Ok(identifier(register)),
            Token::Literal(Literal::Number(address)) => Ok(absolute(address)),
            Token::Plus => relative(parser, true),
            Token::Minus => relative(parser, false),
            t => Err(ParseError::UnexpectedToken(t)),
        }
    }
}

fn literal(parser: &mut Parser) -> Result<Operand, ParseError> {
    let token = parser.next()?;

    match token {
        Token::Literal(Literal::Number(num)) => Ok(Operand::Literal(num)),
        Token::LeftCurlyBracket => constant(parser, true),
        t => Err(ParseError::UnexpectedToken(t.clone()))
    }
}

fn indirect(parser: &mut Parser) -> Result<Operand, ParseError> {
    let operand = Operand::parse(parser)?;
    parser.expect::<RightBracket>()?;

    match operand {
        Operand::Register(register) => Ok(register_indirect(register)),
        operand => Err(ParseError::UnsupportedIndirect(operand)),
    }
}

fn register_indirect(register: RegisterId) -> Operand {
    Operand::Indirect(OperandIndirect::Register(register))
}

fn identifier(identifier: String) -> Operand {
    match get_register(&identifier) {
        Some(reg) => Operand::Register(reg),
        None => Operand::Label(identifier),
    }
}

fn absolute(address: u32) -> Operand {
    Operand::Address(Address::Absolute(address))
}

fn relative(parser: &mut Parser, forward: bool) -> Result<Operand, ParseError> {
    let address: Number = parser.expect()?;
    #[allow(clippy::cast_possible_wrap)]
    let address = address.0 as i32;

    let address = if forward {
        address
    } else {
        -address
    };

    Ok(Operand::Address(Address::Relative(address)))
}

fn constant(parser: &mut Parser, is_literal: bool) -> Result<Operand, ParseError> {
    let constant: Identifier = parser.expect::<Identifier>()?;
    parser.expect::<RightCurlyBracket>()?;

    let operand = if is_literal {
        Operand::ConstantLiteral(constant.0)
    } else {
        Operand::ConstantAddress(constant.0)
    };

    Ok(operand)
}

fn get_register(register: &str) -> Option<RegisterId> {
    let register = register.to_ascii_uppercase();

    match register.as_str() {
        "R0" => Some(RegisterId::R0),
        "R1" => Some(RegisterId::R1),
        "R2" => Some(RegisterId::R2),
        "R3" => Some(RegisterId::R3),
        "R4" => Some(RegisterId::R4),
        "R5" => Some(RegisterId::R5),
        "R6" => Some(RegisterId::R6),
        "R7" => Some(RegisterId::R7),
        "R8" => Some(RegisterId::R8),
        "R9" => Some(RegisterId::R9),
        "R10" => Some(RegisterId::R10),
        "R11" => Some(RegisterId::R11),
        "R12" => Some(RegisterId::R12),
        "R13" => Some(RegisterId::R13),
        "R14" => Some(RegisterId::R14),
        _ => None
    }
}
