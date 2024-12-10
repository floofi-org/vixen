use crate::models::token::Literal;
use crate::scanner::Scanner;

use super::Token;

const FORBIDDEN_LITERAL_CHARS: &[char] = &[':', '$', '#', '%', '-', '+', '.', ',', ';'];

pub fn number(scanner: &mut Scanner, radix: u32) -> Option<Token> {
    let number = scanner.next_while(literal_filter)?;
    let number = u32::from_str_radix(&number, radix)
        .expect("Invalid number literal");

    Some(Token::Literal(Literal::Number(number)))
}

pub fn identifier(scanner: &mut Scanner) -> Option<Token> {
    let identifier = scanner.next_while(literal_filter)?;

    Some(Token::Literal(Literal::Identifier(identifier)))
}

// .next_while require we take a reference of char
#[allow(clippy::trivially_copy_pass_by_ref)]
fn literal_filter(char: &char) -> bool {
    !(char.is_whitespace() || FORBIDDEN_LITERAL_CHARS.contains(char))
}
