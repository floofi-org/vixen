use crate::models::escape_char;
use crate::models::token::Literal;
use crate::scanner::Scanner;

use super::Token;

const FORBIDDEN_LITERAL_CHARS: &[char] = &[
    ':', '$', '#', '%',
    '-', '+', '[', ']',
    '.', ',', ';', '\'',
    '"', '=', '{', '}',
    '\\',
];

pub fn number(scanner: &mut Scanner, radix: u32) -> Option<Token> {
    let number = scanner.next_while(literal_filter)?;
    let number = u32::from_str_radix(&number, radix)
        .expect("Invalid number literal");

    Some(Token::Literal(Literal::Number(number)))
}

pub fn number_char(scanner: &mut Scanner) -> Token {
    let mut char = scanner.next()
        .filter(char::is_ascii)
        .expect("Invalid character literal");

    if char == '\\' {
        char = scanner.next()
            .map(escape_char)
            .expect("Unexpected EOF on char escape code");
    }

    let char = char as u32;

    Token::Literal(Literal::Number(char))
}

pub fn identifier(scanner: &mut Scanner) -> Option<Token> {
    let identifier = scanner.next_while(literal_filter)?;

    Some(Token::Literal(Literal::Identifier(identifier)))
}

pub fn string(scanner: &mut Scanner) -> Option<Token> {
    // .next_while require we take a reference of char
    #[allow(clippy::trivially_copy_pass_by_ref)]
    fn quotation_mark(char: &char) -> bool {
        *char == '"'
    }

    scanner.next()
        .filter(quotation_mark)
        .expect("Expected beginning of a string");

    let string = scanner.next_while(|c| !quotation_mark(c))?;
    scanner.next()
        .filter(quotation_mark)
        .expect("Expected end of a string");

    let string = escape_string(&string);

    Some(Token::Literal(Literal::String(string)))
}


fn escape_string(string: &str) -> String {
    let mut escaped = String::with_capacity(string.len());

    let mut chars = string.chars();
    while let Some(char) = chars.next() {
        let char = if char == '\\' {
            let char = chars.next().expect("Unexpected end of string on escape sequence");
            escape_char(char)
        } else {
            char
        };

        escaped.push(char);
    }

    escaped
}

// .next_while require we take a reference of char
#[allow(clippy::trivially_copy_pass_by_ref)]
fn literal_filter(char: &char) -> bool {
    !(char.is_whitespace() || FORBIDDEN_LITERAL_CHARS.contains(char))
}
