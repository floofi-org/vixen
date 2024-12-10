use vixen::core::registers::RegisterId;

use crate::models::Token;
use crate::models::token::{Literal, Number};

use super::{FromTokenStream, ParseError, Parser};

#[derive(Debug)]
pub enum Operand {
    Literal(u32),
    Register(RegisterId),
    Address(Address),
}

#[derive(Debug)]
enum Address {
    Absolute(u32),
    Relative(i32)
}

impl FromTokenStream for Operand {
    fn parse(parser: &mut Parser) -> Result<Self, ParseError> {
        let token = parser.next()?;

        match token {
            Token::Hash => immediate(parser),
            Token::Literal(Literal::Identifier(register)) => direct(register),
            Token::Literal(Literal::Number(address)) => Ok(absolute(address)),
            Token::Plus => relative(parser, true),
            Token::Minus => relative(parser, false),
            _ => Err(ParseError::UnexpectedToken),
        }
    }
}

fn immediate(parser: &mut Parser) -> Result<Operand, ParseError> {
    let number: Number = parser.expect()?;

    Ok(Operand::Literal(number.0))
}

fn direct(mut register: String) -> Result<Operand, ParseError> {
    register.make_ascii_lowercase();

    let Some(register) = register.strip_prefix('r') else {
        return Err(ParseError::InvalidOperand("Registers must start with R prefix"));
    };

    let Ok(register) = register.parse::<u32>() else {
        return Err(ParseError::InvalidOperand("Failed parsing regsiter number"));
    };

    let Some(register_id) = get_register(register) else {
        return Err(ParseError::InvalidOperand("No such register"));
    };

    Ok(Operand::Register(register_id))
}

fn absolute(address: u32) -> Operand {
    Operand::Address(Address::Absolute(address))
}

fn relative(parser: &mut Parser, forward: bool) -> Result<Operand, ParseError> {
    let address: Number = parser.expect()?;
    let address = address.0 as i32;

    let address = if forward {
        address
    } else {
        -address
    };

    Ok(Operand::Address(Address::Relative(address)))
}

// FIXME: AAAAAAAAAAAAAAAA
fn get_register(register: u32) -> Option<RegisterId> {
    match register {
        0 => Some(RegisterId::R0),
        1 => Some(RegisterId::R1),
        2 => Some(RegisterId::R2),
        3 => Some(RegisterId::R3),
        4 => Some(RegisterId::R4),
        5 => Some(RegisterId::R5),
        6 => Some(RegisterId::R6),
        7 => Some(RegisterId::R7),
        8 => Some(RegisterId::R8),
        9 => Some(RegisterId::R9),
        10 => Some(RegisterId::R10),
        11 => Some(RegisterId::R11),
        12 => Some(RegisterId::R12),
        13 => Some(RegisterId::R13),
        14 => Some(RegisterId::R14),
        _ => None
    }
}
