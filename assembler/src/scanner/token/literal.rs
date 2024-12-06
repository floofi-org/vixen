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
    /// # Panics
    /// Panics when parsing number fails
    pub fn number(scanner: &mut Scanner, radix: u32) -> Option<Self> {
        let number = scanner.next_while(literal_filter)?;
        let number = u16::from_str_radix(&number, radix)
            .expect("Your number is silly");

        Some(Self::Number(number))
    }

    pub fn identifier(scanner: &mut Scanner) -> Option<Self> {
        let identifier = scanner.next_while(literal_filter)?;

        Some(Self::Identifier(identifier))
    }
}

fn literal_filter(char: &char) -> bool {
    !(char.is_whitespace() || FORBIDDEN_LITERAL_CHARS.contains(char))
}
