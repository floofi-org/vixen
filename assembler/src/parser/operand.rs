use vixen::core::registers::RegisterId;

use crate::models::{Address, Operand, Token};
use crate::models::token::{Literal, Number};

use super::{FromTokenStream, ParseError, Parser};

impl FromTokenStream for Operand {
    fn parse(parser: &mut Parser) -> Result<Self, ParseError> {
        let token = parser.next()?;

        match token {
            Token::Hash => literal(parser),
            Token::Literal(Literal::Identifier(register)) => identifier(register),
            Token::Literal(Literal::Number(address)) => Ok(absolute(address)),
            Token::Plus => relative(parser, true),
            Token::Minus => relative(parser, false),
            t => Err(ParseError::UnexpectedToken(t)),
        }
    }
}

fn literal(parser: &mut Parser) -> Result<Operand, ParseError> {
    let number: Number = parser.expect()?;

    Ok(Operand::Literal(number.0))
}

fn identifier(mut identifier: String) -> Result<Operand, ParseError> {
    identifier.make_ascii_lowercase();

    match identifier.strip_prefix('r') {
        Some(reg) => register(reg),
        None => Ok(Operand::Label(identifier)),
    }
}

fn register(register: &str) -> Result<Operand, ParseError> {
    get_register(register)
        .map(Operand::Register)
        .ok_or(ParseError::InvalidOperand("No such register"))
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
fn get_register(register: &str) -> Option<RegisterId> {
    match register {
        "0" => Some(RegisterId::R0),
        "1" => Some(RegisterId::R1),
        "2" => Some(RegisterId::R2),
        "3" => Some(RegisterId::R3),
        "4" => Some(RegisterId::R4),
        "5" => Some(RegisterId::R5),
        "6" => Some(RegisterId::R6),
        "7" => Some(RegisterId::R7),
        "8" => Some(RegisterId::R8),
        "9" => Some(RegisterId::R9),
        "10" => Some(RegisterId::R10),
        "11" => Some(RegisterId::R11),
        "12" => Some(RegisterId::R12),
        "13" => Some(RegisterId::R13),
        "14" => Some(RegisterId::R14),
        _ => None
    }
}
