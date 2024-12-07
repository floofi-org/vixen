use crate::scanner::Scanner;

const FORBIDDEN_LITERAL_CHARS: &[char] = &[
    ':',
    '$',
    '#',
    '%',
    '-',
    '+',
    '.',
    ',',
    ';',
];

#[derive(Debug)]
pub enum Literal {
    Identifier(String),
    Number(u16),
}

impl Literal {
    pub fn number(scanner: &mut Scanner, radix: u32) -> Option<Self> {
        let number = scanner.next_while(literal_filter)?;
        let number = u16::from_str_radix(&number, radix)
            .expect("Invalid number literal");

        Some(Self::Number(number))
    }

    pub fn identifier(scanner: &mut Scanner) -> Option<Self> {
        let identifier = scanner.next_while(literal_filter)?;

        Some(Self::Identifier(identifier))
    }
}

// .next_while require we take a reference of char
#[allow(clippy::trivially_copy_pass_by_ref)]
fn literal_filter(char: &char) -> bool {
    !(char.is_whitespace() || FORBIDDEN_LITERAL_CHARS.contains(char))
}
